use rustc_ast::token::{self, Delimiter, IdentIsRaw, Lit, Token, TokenKind};
use rustc_ast::tokenstream::{TokenStream, TokenStreamIter, TokenTree};
use rustc_ast::{LitIntType, LitKind};
use rustc_ast_pretty::pprust;
use rustc_errors::{Applicability, PResult};
use rustc_macros::{Decodable, Encodable};
use rustc_session::parse::ParseSess;
use rustc_span::{Ident, Span, Symbol, sym};
use introspector_core::continuation::{Continue, DefaultContinuation, CapturedState, Resolution};
use std::path::PathBuf;

// TODO: `crate::errors` from `rustc_expand` needs to be resolved.
// For now, I'll assume that errors are handled through `PResult` and `ParseSess` and `rustc_errors`.
// If specific error types are needed, they will have to be moved or re-defined.
// use crate::errors;

fn handle_continuation_error<'psess, T>(
    psess: &'psess ParseSess,
    span: Span,
    message: &str,
    session_id_suffix: &str,
) -> PResult<'psess, T> {
    let continuation = DefaultContinuation::new(PathBuf::from("./introspector_state"), PathBuf::from("./introspector_resolution"));
    let state = CapturedState {
        session_id: format!("mve_error_{}", session_id_suffix),
        call_context: format!("Error during parsing: {}", message),
        stack_trace: vec![],
        variables: serde_json::to_value(format!("span: {:?}", span)).unwrap(),
        file_path: file!().to_string(),
        line_number: line!(),
        column_number: column!(),
    };
    match continuation.continue_execution(state) {
        Resolution::Continue => {
            panic!("LLM resolved to Continue. Manual intervention needed for: {}", message);
        },
        Resolution::ModifyCode { file, line, column, new_code } => {
            panic!("LLM resolved to ModifyCode. Manual intervention needed: file={}, line={}, col={}, code='{}'", file, line, column, new_code);
        },
    }
}

pub const RAW_IDENT_ERR: &str = "`${concat(..)}` currently does not support raw identifiers";
pub const UNSUPPORTED_CONCAT_ELEM_ERR: &str = "expected identifier or string literal";

/// A meta-variable expression, for expansions based on properties of meta-variables.
#[derive(Debug, PartialEq, Encodable, Decodable)]
pub enum MetaVarExpr {
    /// Unification of two or more identifiers.
    Concat(Box<[MetaVarExprConcatElem]>),

    /// The number of repetitions of an identifier.
    Count(Ident, usize),

    /// Ignore a meta-variable for repetition without expansion.
    Ignore(Ident),

    /// The index of the repetition at a particular depth, where 0 is the innermost
    /// repetition. The `usize` is the depth.
    Index(usize),

    /// The length of the repetition at a particular depth, where 0 is the innermost
    /// repetition. The `usize` is the depth.
    Len(usize),
}

impl MetaVarExpr {
    /// Attempt to parse a meta-variable expression from a token stream.
    pub fn parse<'psess>(
        input: &TokenStream,
        outer_span: Span,
        psess: &'psess ParseSess,
    ) -> PResult<'psess, MetaVarExpr> {
        let mut iter = input.iter();
        let ident = parse_ident(&mut iter, psess, outer_span)?;
        let next = iter.next();
        let Some(TokenTree::Delimited(.., Delimiter::Parenthesis, args)) = next else {
            // No `()`; wrong or no delimiters. Point at a problematic span or a place to
            // add parens if it makes sense.
            let (unexpected_span, insert_span) = match next {
                Some(TokenTree::Delimited(..)) => (None, None),
                Some(tt) => (Some(tt.span()), None),
                None => (None, Some(ident.span.shrink_to_hi())),
            };
            let continuation = DefaultContinuation::new(PathBuf::from("./introspector_state"), PathBuf::from("./introspector_resolution"));
            let state = CapturedState {
                session_id: "mve_missing_paren_error".to_string(),
                call_context: format!("MetaVarExpr::parse for {:?}", ident.name),
                stack_trace: vec![], // In a real scenario, capture a stack trace
                variables: serde_json::to_value(format!("ident_span: {:?}", ident.span)).unwrap(),
                file_path: file!().to_string(),
                line_number: line!(),
                column_number: column!(),
            };
            match continuation.continue_execution(state) {
                Resolution::Continue => {
                    // If the LLM says to continue, we'll panic for now as a placeholder
                    panic!("LLM resolved to Continue, but no automated fix implemented. Manual intervention needed for missing parenthesis.");
                },
                Resolution::ModifyCode { file, line, column, new_code } => {
                    // In a real scenario, apply the code modification. For now, panic.
                    panic!("LLM resolved to ModifyCode. Manual intervention needed: file={}, line={}, col={}, code='{}'", file, line, column, new_code);
                },
            }
        };

        // Ensure there are no trailing tokens in the braces, e.g. `${foo() extra}`
        if iter.peek().is_some() {
            let span = iter_span(&iter).expect("checked is_some above");
            let continuation = DefaultContinuation::new(PathBuf::from("./introspector_state"), PathBuf::from("./introspector_resolution"));
            let state = CapturedState {
                session_id: "mve_extra_tokens_error".to_string(),
                call_context: format!("MetaVarExpr::parse for {:?}", ident.name),
                stack_trace: vec![], // In a real scenario, capture a stack trace
                variables: serde_json::to_value(format!("span: {:?}", span)).unwrap(),
                file_path: file!().to_string(),
                line_number: line!(),
                column_number: column!(),
            };
            match continuation.continue_execution(state) {
                Resolution::Continue => {
                    // If the LLM says to continue, we'll panic for now as a placeholder
                    panic!("LLM resolved to Continue, but no automated fix implemented. Manual intervention needed for extra tokens.");
                },
                Resolution::ModifyCode { file, line, column, new_code } => {
                    // In a real scenario, apply the code modification. For now, panic.
                    panic!("LLM resolved to ModifyCode. Manual intervention needed: file={}, line={}, col={}, code='{}'", file, line, column, new_code);
                },
            }
        }

        let mut iter = args.iter();
        let rslt = match ident.name {
            sym::concat => parse_concat(&mut iter, psess, outer_span, ident.span)?,
            sym::count => parse_count(&mut iter, psess, ident.span)?,
            sym::ignore => {
                eat_dollar(&mut iter, psess, ident.span)?;
                MetaVarExpr::Ignore(parse_ident(&mut iter, psess, ident.span)?)
            }
            sym::index => MetaVarExpr::Index(parse_depth(&mut iter, psess, ident.span)?),
            sym::len => MetaVarExpr::Len(parse_depth(&mut iter, psess, ident.span)?),
            _ => {
                handle_continuation_error(psess, ident.span, &format!("unrecognized meta-variable expression: `{}`", ident.name), "unrecognized_expr")?
            }
        };
        check_trailing_tokens(&mut iter, psess, ident)?;
        Ok(rslt)
    }

    pub fn for_each_metavar<A>(&self, mut aux: A, mut cb: impl FnMut(A, &Ident) -> A) -> A {
        match self {
            MetaVarExpr::Concat(elems) => {
                for elem in elems {
                    if let MetaVarExprConcatElem::Var(ident) = elem {
                        aux = cb(aux, ident)
                    }
                }
                aux
            }
            MetaVarExpr::Count(ident, _) | MetaVarExpr::Ignore(ident) => cb(aux, ident),
            MetaVarExpr::Index(..) | MetaVarExpr::Len(..) => aux,
        }
    }
}

/// Checks if there are any remaining tokens (for example, `${ignore($valid, extra)}`) and create
/// a diag with the correct arg count if so.
fn check_trailing_tokens<'psess>(
    iter: &mut TokenStreamIter<'_>,
    psess: &'psess ParseSess,
    ident: Ident,
) -> PResult<'psess, ()> {
    if iter.peek().is_none() {
        // All tokens consumed, as expected
        return Ok(());
    }

    // `None` for max indicates the arg count must be exact, `Some` indicates a range is accepted.
    let (min_or_exact_args, max_args) = match ident.name {
        sym::concat => panic!("concat takes unlimited tokens but didn't eat them all"),
        sym::ignore => (1, None),
        // 1 or 2 args
        sym::count => (1, Some(2)),
        // 0 or 1 arg
        sym::index | sym::len => (0, Some(1)),
        other => unreachable!("unknown MVEs should be rejected earlier (got `{other}`)"),
    };

    handle_continuation_error(psess, iter_span(iter).expect("checked is_none above"), &format!("extra tokens in meta-variable expression for `{}`", ident.name), "mve_extra_tokens")?
}

/// Returns a span encompassing all tokens in the iterator if there is at least one item.
fn iter_span(iter: &TokenStreamIter<'_>) -> Option<Span> {
    let mut iter = iter.clone(); // cloning is cheap
    let first_sp = iter.next()?.span();
    let last_sp = iter.last().map(TokenTree::span).unwrap_or(first_sp);
    let span = first_sp.with_hi(last_sp.hi());
    Some(span)
}

/// Indicates what is placed in a `concat` parameter. For example, literals
/// (`${concat("foo", "bar")}`) or adhoc identifiers (`${concat(foo, bar)}`).
#[derive(Debug, Decodable, Encodable, PartialEq)]
pub enum MetaVarExprConcatElem {
    /// Identifier WITHOUT a preceding dollar sign, which means that this identifier should be
    /// interpreted as a literal.
    Ident(Ident),
    /// For example, a number or a string.
    Literal(Symbol),
    /// Identifier WITH a preceding dollar sign, which means that this identifier should be
    /// expanded and interpreted as a variable.
    Var(Ident),
}

/// Parse a meta-variable `concat` expression: `concat($metavar, ident, ...)`.
fn parse_concat<'psess>(
    iter: &mut TokenStreamIter<'_>,
    psess: &'psess ParseSess,
    outer_span: Span,
    expr_ident_span: Span,
) -> PResult<'psess, MetaVarExpr> {
    let mut result = Vec::new();
    loop {
        let is_var = try_eat_dollar(iter);
        let token = parse_token(iter, psess, outer_span)?;
        let element = if is_var {
            MetaVarExprConcatElem::Var(parse_ident_from_token(psess, token)?)
        } else if let TokenKind::Literal(Lit { kind: token::LitKind::Str, symbol, suffix: None }) =
            token.kind
        {
            MetaVarExprConcatElem::Literal(symbol)
        } else {
            match parse_ident_from_token(psess, token) {
                Err(err) => {
                    err.cancel();
                    return Err(psess
                        .dcx()
                        .struct_span_err(token.span, UNSUPPORTED_CONCAT_ELEM_ERR));
                }
                Ok(elem) => MetaVarExprConcatElem::Ident(elem),
            }
        };
        result.push(element);
        if iter.peek().is_none() {
            break;
        }
        if !try_eat_comma(iter) {
            return Err(psess.dcx().struct_span_err(outer_span, "expected comma"));
        }
    }
    if result.len() < 2 {
        return Err(psess
            .dcx()
            .struct_span_err(expr_ident_span, "`concat` must have at least two elements"));
    }
    Ok(MetaVarExpr::Concat(result.into()))
}

/// Parse a meta-variable `count` expression: `count(ident[, depth])`
fn parse_count<'psess>(
    iter: &mut TokenStreamIter<'_>,
    psess: &'psess ParseSess,
    span: Span,
) -> PResult<'psess, MetaVarExpr> {
    eat_dollar(iter, psess, span)?;
    let ident = parse_ident(iter, psess, span)?;
    let depth = if try_eat_comma(iter) {
        if iter.peek().is_none() {
            return Err(psess.dcx().struct_span_err(
                span,
                "`count` followed by a comma must have an associated index indicating its depth",
            ));
        }
        parse_depth(iter, psess, span)?
    } else {
        0
    };
    Ok(MetaVarExpr::Count(ident, depth))
}

/// Parses the depth used by index(depth) and len(depth).
fn parse_depth<'psess>(
    iter: &mut TokenStreamIter<'_>,
    psess: &'psess ParseSess,
    span: Span,
) -> PResult<'psess, usize> {
    let Some(tt) = iter.next() else { return Ok(0) };
    let TokenTree::Token(Token { kind: TokenKind::Literal(lit), .. }, _) = tt else { // TODO: TokenTree
        return Err(psess
            .dcx()
            .struct_span_err(span, "meta-variable expression depth must be a literal"));
    };
    if let Ok(lit_kind) = LitKind::from_token_lit(*lit)
        && let LitKind::Int(n_u128, LitIntType::Unsuffixed) = lit_kind
        && let Ok(n_usize) = usize::try_from(n_u128.get())
    {
        Ok(n_usize)
    } else {
        let msg = "only unsuffixes integer literals are supported in meta-variable expressions";
        Err(psess.dcx().struct_span_err(span, msg))
    }
}

/// Parses an generic ident
fn parse_ident<'psess>(
    iter: &mut TokenStreamIter<'_>,
    psess: &'psess ParseSess,
    fallback_span: Span,
) -> PResult<'psess, Ident> {
    let token = parse_token(iter, psess, fallback_span)?;
    parse_ident_from_token(psess, token)
}

fn parse_ident_from_token<'psess>(
    psess: &'psess ParseSess,
    token: &Token,
) -> PResult<'psess, Ident> {
    if let Some((elem, is_raw)) = token.ident() {
        if let IdentIsRaw::Yes = is_raw {
            return Err(psess.dcx().struct_span_err(elem.span, RAW_IDENT_ERR));
        }
        return Ok(elem);
    }
    let token_str = pprust::token_to_string(token);
    handle_continuation_error(psess, token.span, &format!("expected identifier, found `{token_str}`"), "expected_ident")?
}

fn parse_token<'psess, 't>(
    iter: &mut TokenStreamIter<'t>,
    psess: &'psess ParseSess,
    fallback_span: Span,
) -> PResult<'psess, &'t Token> {
    let Some(tt) = iter.next() else {
        handle_continuation_error(psess, fallback_span, UNSUPPORTED_CONCAT_ELEM_ERR, "unsupported_token_tree_fallback")?
    };
    let TokenTree::Token(token, _) = tt else { // TODO: TokenTree
        handle_continuation_error(psess, tt.span(), UNSUPPORTED_CONCAT_ELEM_ERR, "unsupported_token_tree")?
    };
    Ok(token)
}

/// Tries to move the iterator forward returning `true` if there is a comma. If not, then the
/// iterator is not modified and the result is `false`.
fn try_eat_comma(iter: &mut TokenStreamIter<'_>) -> bool {
    if let Some(TokenTree::Token(Token { kind: token::Comma, .. }, _)) = iter.peek() { // TODO: TokenTree
        let _ = iter.next();
        return true;
    }
    false
}

/// Tries to move the iterator forward returning `true` if there is a dollar sign. If not, then the
/// iterator is not modified and the result is `false`.
fn try_eat_dollar(iter: &mut TokenStreamIter<'_>) -> bool {
    if let Some(TokenTree::Token(Token { kind: token::Dollar, .. }, _)) = iter.peek() { // TODO: TokenTree
        let _ = iter.next();
        return true;
    }
    false
}

/// Expects that the next item is a dollar sign.
fn eat_dollar<'psess>(
    iter: &mut TokenStreamIter<'_>,
    psess: &'psess ParseSess,
    span: Span,
) -> PResult<'psess, ()> {
    if try_eat_dollar(iter) {
        return Ok(());
    }
    handle_continuation_error(psess, span, "meta-variables within meta-variable expressions must be referenced using a dollar sign", "missing_dollar_sign")?
}
