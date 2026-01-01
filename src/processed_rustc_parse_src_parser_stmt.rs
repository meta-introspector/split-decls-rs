/* FP:stmt.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_parser_stmt_UNPARSEABLE_0001
/* FP:stmt.rs-0002 */ use std::borrow::Cow;
/* FP:stmt.rs-0003 */ use std::mem;
/* FP:stmt.rs-0004 */ use std::ops::Bound;
/* FP:stmt.rs-0005 */ 
/* FP:stmt.rs-0006 */ use ast::Label;
/* FP:stmt.rs-0007 */ use rustc_ast as ast;
/* FP:stmt.rs-0008 */ use crate::rustc_complete::token::{self, Delimiter, InvisibleOrigin, MetaVarKind, TokenKind};
/* FP:stmt.rs-0009 */ use crate::rustc_complete::util::classify::{self, TrailingBrace};
/* FP:stmt.rs-0010 */ use crate::rustc_complete::visit::{Visitor, walk_expr};
/* FP:stmt.rs-0011 */ use crate::rustc_complete::{
/* FP:stmt.rs-0012 */     AttrStyle, AttrVec, Block, BlockCheckMode, DUMMY_NODE_ID, Expr, ExprKind, HasAttrs, Local,
/* FP:stmt.rs-0013 */     LocalKind, MacCall, MacCallStmt, MacStmtStyle, Recovered, Stmt, StmtKind,
/* FP:stmt.rs-0014 */ };
/* FP:stmt.rs-0015 */ use crate::rustc_complete::{Applicability, Diag, PResult};
/* FP:stmt.rs-0016 */ use crate::rustc_complete::{BytePos, ErrorGuaranteed, Ident, Span, kw, sym};
/* FP:stmt.rs-0017 */ use thin_vec::{ThinVec, thin_vec};
/* FP:stmt.rs-0018 */ 
/* FP:stmt.rs-0019 */ use super::attr::InnerAttrForbiddenReason;
/* FP:stmt.rs-0020 */ use super::diagnostics::AttemptLocalParseRecovery;
/* FP:stmt.rs-0021 */ use super::pat::{PatternLocation, RecoverComma};
/* FP:stmt.rs-0022 */ use super::path::PathStyle;
/* FP:stmt.rs-0023 */ use super::{
/* FP:stmt.rs-0024 */     AttrWrapper, BlockMode, FnContext, FnParseMode, ForceCollect, Parser, Restrictions,
/* FP:stmt.rs-0025 */     SemiColonMode, Trailing, UsePreAttrPos,
/* FP:stmt.rs-0026 */ };
/* FP:stmt.rs-0027 */ use crate::errors::{self, MalformedLoopLabel};
/* FP:stmt.rs-0028 */ use crate::exp;
/* FP:stmt.rs-0029 */ 
/* FP:stmt.rs-0030 */ impl<'a> Parser<'a> {
/* FP:stmt.rs-0031 */     /// Parses a statement. This stops just before trailing semicolons on everything but items.
/* FP:stmt.rs-0032 */     /// e.g., a `StmtKind::Semi` parses to a `StmtKind::Expr`, leaving the trailing `;` unconsumed.
/* FP:stmt.rs-0033 */     ///
/* FP:stmt.rs-0034 */     /// If `force_collect` is [`ForceCollect::Yes`], forces collection of tokens regardless of
/* FP:stmt.rs-0035 */     /// whether or not we have attributes.
/* FP:stmt.rs-0036 */     // Public for rustfmt usage.
/* FP:stmt.rs-0037 */     pub fn parse_stmt(&mut self, force_collect: ForceCollect) -> PResult<'a, Option<Stmt>> {
/* FP:stmt.rs-0038 */         Ok(self.parse_stmt_without_recovery(false, force_collect, false).unwrap_or_else(|e| {
/* FP:stmt.rs-0039 */             e.emit();
/* FP:stmt.rs-0040 */             self.recover_stmt_(SemiColonMode::Break, BlockMode::Ignore);
/* FP:stmt.rs-0041 */             None
/* FP:stmt.rs-0042 */         }))
/* FP:stmt.rs-0043 */     }
/* FP:stmt.rs-0044 */ 
/* FP:stmt.rs-0045 */     /// If `force_collect` is [`ForceCollect::Yes`], forces collection of tokens regardless of
/* FP:stmt.rs-0046 */     /// whether or not we have attributes. If `force_full_expr` is true, parses the stmt without
/* FP:stmt.rs-0047 */     /// using `Restriction::STMT_EXPR`. Public for `cfg_eval` macro expansion.
/* FP:stmt.rs-0048 */     pub fn parse_stmt_without_recovery(
/* FP:stmt.rs-0049 */         &mut self,
/* FP:stmt.rs-0050 */         capture_semi: bool,
/* FP:stmt.rs-0051 */         force_collect: ForceCollect,
/* FP:stmt.rs-0052 */         force_full_expr: bool,
/* FP:stmt.rs-0053 */     ) -> PResult<'a, Option<Stmt>> {
/* FP:stmt.rs-0054 */         let pre_attr_pos = self.collect_pos();
/* FP:stmt.rs-0055 */         let attrs = self.parse_outer_attributes()?;
/* FP:stmt.rs-0056 */         let lo = self.token.span;
/* FP:stmt.rs-0057 */ 
/* FP:stmt.rs-0058 */         if let Some(stmt) = self.eat_metavar_seq(MetaVarKind::Stmt, |this| {
/* FP:stmt.rs-0059 */             this.parse_stmt_without_recovery(false, ForceCollect::Yes, false)
/* FP:stmt.rs-0060 */         }) {
/* FP:stmt.rs-0061 */             let mut stmt = stmt.expect("an actual statement");
/* FP:stmt.rs-0062 */             stmt.visit_attrs(|stmt_attrs| {
/* FP:stmt.rs-0063 */                 attrs.prepend_to_nt_inner(stmt_attrs);
/* FP:stmt.rs-0064 */             });
/* FP:stmt.rs-0065 */             return Ok(Some(stmt));
/* FP:stmt.rs-0066 */         }
/* FP:stmt.rs-0067 */ 
/* FP:stmt.rs-0068 */         if self.token.is_keyword(kw::Mut) && self.is_keyword_ahead(1, &[kw::Let]) {
/* FP:stmt.rs-0069 */             self.bump();
/* FP:stmt.rs-0070 */             let mut_let_span = lo.to(self.token.span);
/* FP:stmt.rs-0071 */             self.dcx().emit_err(errors::InvalidVariableDeclaration {
/* FP:stmt.rs-0072 */                 span: mut_let_span,
/* FP:stmt.rs-0073 */                 sub: errors::InvalidVariableDeclarationSub::SwitchMutLetOrder(mut_let_span),
/* FP:stmt.rs-0074 */             });
/* FP:stmt.rs-0075 */         }
/* FP:stmt.rs-0076 */ 
/* FP:stmt.rs-0077 */         let stmt = if self.token.is_keyword(kw::Super) && self.is_keyword_ahead(1, &[kw::Let]) {
/* FP:stmt.rs-0078 */             self.collect_tokens(None, attrs, force_collect, |this, attrs| {
/* FP:stmt.rs-0079 */                 let super_span = this.token.span;
/* FP:stmt.rs-0080 */                 this.expect_keyword(exp!(Super))?;
/* FP:stmt.rs-0081 */                 this.expect_keyword(exp!(Let))?;
/* FP:stmt.rs-0082 */                 this.psess.gated_spans.gate(sym::super_let, super_span);
/* FP:stmt.rs-0083 */                 let local = this.parse_local(Some(super_span), attrs)?;
/* FP:stmt.rs-0084 */                 let trailing = Trailing::from(capture_semi && this.token == token::Semi);
/* FP:stmt.rs-0085 */                 Ok((
/* FP:stmt.rs-0086 */                     this.mk_stmt(lo.to(this.prev_token.span), StmtKind::Let(local)),
/* FP:stmt.rs-0087 */                     trailing,
/* FP:stmt.rs-0088 */                     UsePreAttrPos::No,
/* FP:stmt.rs-0089 */                 ))
/* FP:stmt.rs-0090 */             })?
/* FP:stmt.rs-0091 */         } else if self.token.is_keyword(kw::Let) {
/* FP:stmt.rs-0092 */             self.collect_tokens(None, attrs, force_collect, |this, attrs| {
/* FP:stmt.rs-0093 */                 this.expect_keyword(exp!(Let))?;
/* FP:stmt.rs-0094 */                 let local = this.parse_local(None, attrs)?;
/* FP:stmt.rs-0095 */                 let trailing = Trailing::from(capture_semi && this.token == token::Semi);
/* FP:stmt.rs-0096 */                 Ok((
/* FP:stmt.rs-0097 */                     this.mk_stmt(lo.to(this.prev_token.span), StmtKind::Let(local)),
/* FP:stmt.rs-0098 */                     trailing,
/* FP:stmt.rs-0099 */                     UsePreAttrPos::No,
/* FP:stmt.rs-0100 */                 ))
/* FP:stmt.rs-0101 */             })?
/* FP:stmt.rs-0102 */         } else if self.is_kw_followed_by_ident(kw::Mut) && self.may_recover() {
/* FP:stmt.rs-0103 */             self.recover_stmt_local_after_let(
/* FP:stmt.rs-0104 */                 lo,
/* FP:stmt.rs-0105 */                 attrs,
/* FP:stmt.rs-0106 */                 errors::InvalidVariableDeclarationSub::MissingLet,
/* FP:stmt.rs-0107 */                 force_collect,
/* FP:stmt.rs-0108 */             )?
/* FP:stmt.rs-0109 */         } else if self.is_kw_followed_by_ident(kw::Auto) && self.may_recover() {
/* FP:stmt.rs-0110 */             self.bump(); // `auto`
/* FP:stmt.rs-0111 */             self.recover_stmt_local_after_let(
/* FP:stmt.rs-0112 */                 lo,
/* FP:stmt.rs-0113 */                 attrs,
/* FP:stmt.rs-0114 */                 errors::InvalidVariableDeclarationSub::UseLetNotAuto,
/* FP:stmt.rs-0115 */                 force_collect,
/* FP:stmt.rs-0116 */             )?
/* FP:stmt.rs-0117 */         } else if self.is_kw_followed_by_ident(sym::var) && self.may_recover() {
/* FP:stmt.rs-0118 */             self.bump(); // `var`
/* FP:stmt.rs-0119 */             self.recover_stmt_local_after_let(
/* FP:stmt.rs-0120 */                 lo,
/* FP:stmt.rs-0121 */                 attrs,
/* FP:stmt.rs-0122 */                 errors::InvalidVariableDeclarationSub::UseLetNotVar,
/* FP:stmt.rs-0123 */                 force_collect,
/* FP:stmt.rs-0124 */             )?
/* FP:stmt.rs-0125 */         } else if self.check_path()
/* FP:stmt.rs-0126 */             && !self.token.is_qpath_start()
/* FP:stmt.rs-0127 */             && !self.is_path_start_item()
/* FP:stmt.rs-0128 */             && !self.is_builtin()
/* FP:stmt.rs-0129 */         {
/* FP:stmt.rs-0130 */             // We have avoided contextual keywords like `union`, items with `crate` visibility,
/* FP:stmt.rs-0131 */             // or `auto trait` items. We aim to parse an arbitrary path `a::b` but not something
/* FP:stmt.rs-0132 */             // that starts like a path (1 token), but it fact not a path.
/* FP:stmt.rs-0133 */             // Also, we avoid stealing syntax from `parse_item_`.
/* FP:stmt.rs-0134 */             //
/* FP:stmt.rs-0135 */             // `UsePreAttrPos::Yes` here means the attribute belongs unconditionally to the
/* FP:stmt.rs-0136 */             // expression, not the statement. (But the statement attributes/tokens are obtained
/* FP:stmt.rs-0137 */             // from the expression anyway, because `Stmt` delegates `HasAttrs`/`HasTokens` to
/* FP:stmt.rs-0138 */             // the things within `StmtKind`.)
/* FP:stmt.rs-0139 */             let stmt = self.collect_tokens(
/* FP:stmt.rs-0140 */                 Some(pre_attr_pos),
/* FP:stmt.rs-0141 */                 AttrWrapper::empty(),
/* FP:stmt.rs-0142 */                 force_collect,
/* FP:stmt.rs-0143 */                 |this, _empty_attrs| {
/* FP:stmt.rs-0144 */                     Ok((this.parse_stmt_path_start(lo, attrs)?, Trailing::No, UsePreAttrPos::Yes))
/* FP:stmt.rs-0145 */                 },
/* FP:stmt.rs-0146 */             );
/* FP:stmt.rs-0147 */             match stmt {
/* FP:stmt.rs-0148 */                 Ok(stmt) => stmt,
/* FP:stmt.rs-0149 */                 Err(mut err) => {
/* FP:stmt.rs-0150 */                     self.suggest_add_missing_let_for_stmt(&mut err);
/* FP:stmt.rs-0151 */                     return Err(err);
/* FP:stmt.rs-0152 */                 }
/* FP:stmt.rs-0153 */             }
/* FP:stmt.rs-0154 */         } else if let Some(item) = self.parse_item_common(
/* FP:stmt.rs-0155 */             attrs.clone(), // FIXME: unwanted clone of attrs
/* FP:stmt.rs-0156 */             false,
/* FP:stmt.rs-0157 */             true,
/* FP:stmt.rs-0158 */             FnParseMode { req_name: |_| true, context: FnContext::Free, req_body: true },
/* FP:stmt.rs-0159 */             force_collect,
/* FP:stmt.rs-0160 */         )? {
/* FP:stmt.rs-0161 */             self.mk_stmt(lo.to(item.span), StmtKind::Item(Box::new(item)))
/* FP:stmt.rs-0162 */         } else if self.eat(exp!(Semi)) {
/* FP:stmt.rs-0163 */             // Do not attempt to parse an expression if we're done here.
/* FP:stmt.rs-0164 */             self.error_outer_attrs(attrs);
/* FP:stmt.rs-0165 */             self.mk_stmt(lo, StmtKind::Empty)
/* FP:stmt.rs-0166 */         } else if self.token != token::CloseBrace {
/* FP:stmt.rs-0167 */             // Remainder are line-expr stmts. This is similar to the `parse_stmt_path_start` case
/* FP:stmt.rs-0168 */             // above.
/* FP:stmt.rs-0169 */             let restrictions =
/* FP:stmt.rs-0170 */                 if force_full_expr { Restrictions::empty() } else { Restrictions::STMT_EXPR };
/* FP:stmt.rs-0171 */             let e = self.collect_tokens(
/* FP:stmt.rs-0172 */                 Some(pre_attr_pos),
/* FP:stmt.rs-0173 */                 AttrWrapper::empty(),
/* FP:stmt.rs-0174 */                 force_collect,
/* FP:stmt.rs-0175 */                 |this, _empty_attrs| {
/* FP:stmt.rs-0176 */                     let (expr, _) = this.parse_expr_res(restrictions, attrs)?;
/* FP:stmt.rs-0177 */                     Ok((expr, Trailing::No, UsePreAttrPos::Yes))
/* FP:stmt.rs-0178 */                 },
/* FP:stmt.rs-0179 */             )?;
/* FP:stmt.rs-0180 */             if matches!(e.kind, ExprKind::Assign(..)) && self.eat_keyword(exp!(Else)) {
/* FP:stmt.rs-0181 */                 let bl = self.parse_block()?;
/* FP:stmt.rs-0182 */                 // Destructuring assignment ... else.
/* FP:stmt.rs-0183 */                 // This is not allowed, but point it out in a nice way.
/* FP:stmt.rs-0184 */                 self.dcx().emit_err(errors::AssignmentElseNotAllowed { span: e.span.to(bl.span) });
/* FP:stmt.rs-0185 */             }
/* FP:stmt.rs-0186 */             self.mk_stmt(lo.to(e.span), StmtKind::Expr(e))
/* FP:stmt.rs-0187 */         } else {
/* FP:stmt.rs-0188 */             self.error_outer_attrs(attrs);
/* FP:stmt.rs-0189 */             return Ok(None);
/* FP:stmt.rs-0190 */         };
/* FP:stmt.rs-0191 */ 
/* FP:stmt.rs-0192 */         self.maybe_augment_stashed_expr_in_pats_with_suggestions(&stmt);
/* FP:stmt.rs-0193 */         Ok(Some(stmt))
/* FP:stmt.rs-0194 */     }
/* FP:stmt.rs-0195 */ 
/* FP:stmt.rs-0196 */     fn parse_stmt_path_start(&mut self, lo: Span, attrs: AttrWrapper) -> PResult<'a, Stmt> {
/* FP:stmt.rs-0197 */         let stmt = self.collect_tokens(None, attrs, ForceCollect::No, |this, attrs| {
/* FP:stmt.rs-0198 */             let path = this.parse_path(PathStyle::Expr)?;
/* FP:stmt.rs-0199 */ 
/* FP:stmt.rs-0200 */             if this.eat(exp!(Bang)) {
/* FP:stmt.rs-0201 */                 let stmt_mac = this.parse_stmt_mac(lo, attrs, path)?;
/* FP:stmt.rs-0202 */                 return Ok((
/* FP:stmt.rs-0203 */                     stmt_mac,
/* FP:stmt.rs-0204 */                     Trailing::from(this.token == token::Semi),
/* FP:stmt.rs-0205 */                     UsePreAttrPos::No,
/* FP:stmt.rs-0206 */                 ));
/* FP:stmt.rs-0207 */             }
/* FP:stmt.rs-0208 */ 
/* FP:stmt.rs-0209 */             let expr = if this.eat(exp!(OpenBrace)) {
/* FP:stmt.rs-0210 */                 this.parse_expr_struct(None, path, true)?
/* FP:stmt.rs-0211 */             } else {
/* FP:stmt.rs-0212 */                 let hi = this.prev_token.span;
/* FP:stmt.rs-0213 */                 this.mk_expr(lo.to(hi), ExprKind::Path(None, path))
/* FP:stmt.rs-0214 */             };
/* FP:stmt.rs-0215 */ 
/* FP:stmt.rs-0216 */             let expr = this.with_res(Restrictions::STMT_EXPR, |this| {
/* FP:stmt.rs-0217 */                 this.parse_expr_dot_or_call_with(attrs, expr, lo)
/* FP:stmt.rs-0218 */             })?;
/* FP:stmt.rs-0219 */             // `DUMMY_SP` will get overwritten later in this function
/* FP:stmt.rs-0220 */             Ok((
/* FP:stmt.rs-0221 */                 this.mk_stmt(crate::rustc_span::DUMMY_SP, StmtKind::Expr(expr)),
/* FP:stmt.rs-0222 */                 Trailing::No,
/* FP:stmt.rs-0223 */                 UsePreAttrPos::No,
/* FP:stmt.rs-0224 */             ))
/* FP:stmt.rs-0225 */         })?;
/* FP:stmt.rs-0226 */ 
/* FP:stmt.rs-0227 */         if let StmtKind::Expr(expr) = stmt.kind {
/* FP:stmt.rs-0228 */             // Perform this outside of the `collect_tokens` closure, since our
/* FP:stmt.rs-0229 */             // outer attributes do not apply to this part of the expression.
/* FP:stmt.rs-0230 */             let (expr, _) = self.with_res(Restrictions::STMT_EXPR, |this| {
/* FP:stmt.rs-0231 */                 this.parse_expr_assoc_rest_with(Bound::Unbounded, true, expr)
/* FP:stmt.rs-0232 */             })?;
/* FP:stmt.rs-0233 */             Ok(self.mk_stmt(lo.to(self.prev_token.span), StmtKind::Expr(expr)))
/* FP:stmt.rs-0234 */         } else {
/* FP:stmt.rs-0235 */             Ok(stmt)
/* FP:stmt.rs-0236 */         }
/* FP:stmt.rs-0237 */     }
/* FP:stmt.rs-0238 */ 
/* FP:stmt.rs-0239 */     /// Parses a statement macro `mac!(args)` provided a `path` representing `mac`.
/* FP:stmt.rs-0240 */     /// At this point, the `!` token after the path has already been eaten.
/* FP:stmt.rs-0241 */     fn parse_stmt_mac(&mut self, lo: Span, attrs: AttrVec, path: ast::Path) -> PResult<'a, Stmt> {
/* FP:stmt.rs-0242 */         let args = self.parse_delim_args()?;
/* FP:stmt.rs-0243 */         let hi = self.prev_token.span;
/* FP:stmt.rs-0244 */ 
/* FP:stmt.rs-0245 */         let style = match args.delim {
/* FP:stmt.rs-0246 */             Delimiter::Brace => MacStmtStyle::Braces,
/* FP:stmt.rs-0247 */             _ => MacStmtStyle::NoBraces,
/* FP:stmt.rs-0248 */         };
/* FP:stmt.rs-0249 */ 
/* FP:stmt.rs-0250 */         let mac = Box::new(MacCall { path, args });
/* FP:stmt.rs-0251 */ 
/* FP:stmt.rs-0252 */         let kind = if (style == MacStmtStyle::Braces
/* FP:stmt.rs-0253 */             && !matches!(self.token.kind, token::Dot | token::Question))
/* FP:stmt.rs-0254 */             || matches!(
/* FP:stmt.rs-0255 */                 self.token.kind,
/* FP:stmt.rs-0256 */                 token::Semi
/* FP:stmt.rs-0257 */                     | token::Eof
/* FP:stmt.rs-0258 */                     | token::CloseInvisible(InvisibleOrigin::MetaVar(MetaVarKind::Stmt))
/* FP:stmt.rs-0259 */             ) {
/* FP:stmt.rs-0260 */             StmtKind::MacCall(Box::new(MacCallStmt { mac, style, attrs, tokens: None }))
/* FP:stmt.rs-0261 */         } else {
/* FP:stmt.rs-0262 */             // Since none of the above applied, this is an expression statement macro.
/* FP:stmt.rs-0263 */             let e = self.mk_expr(lo.to(hi), ExprKind::MacCall(mac));
/* FP:stmt.rs-0264 */             let e = self.maybe_recover_from_bad_qpath(e)?;
/* FP:stmt.rs-0265 */             let e = self.parse_expr_dot_or_call_with(attrs, e, lo)?;
/* FP:stmt.rs-0266 */             let (e, _) = self.parse_expr_assoc_rest_with(Bound::Unbounded, false, e)?;
/* FP:stmt.rs-0267 */             StmtKind::Expr(e)
/* FP:stmt.rs-0268 */         };
/* FP:stmt.rs-0269 */         Ok(self.mk_stmt(lo.to(hi), kind))
/* FP:stmt.rs-0270 */     }
/* FP:stmt.rs-0271 */ 
/* FP:stmt.rs-0272 */     /// Error on outer attributes in this context.
/* FP:stmt.rs-0273 */     /// Also error if the previous token was a doc comment.
/* FP:stmt.rs-0274 */     fn error_outer_attrs(&self, attrs: AttrWrapper) {
/* FP:stmt.rs-0275 */         if !attrs.is_empty()
/* FP:stmt.rs-0276 */             && let attrs @ [.., last] = &*attrs.take_for_recovery(self.psess)
/* FP:stmt.rs-0277 */         {
/* FP:stmt.rs-0278 */             if last.is_doc_comment() {
/* FP:stmt.rs-0279 */                 self.dcx().emit_err(errors::DocCommentDoesNotDocumentAnything {
/* FP:stmt.rs-0280 */                     span: last.span,
/* FP:stmt.rs-0281 */                     missing_comma: None,
/* FP:stmt.rs-0282 */                 });
/* FP:stmt.rs-0283 */             } else if attrs.iter().any(|a| a.style == AttrStyle::Outer) {
/* FP:stmt.rs-0284 */                 self.dcx().emit_err(errors::ExpectedStatementAfterOuterAttr { span: last.span });
/* FP:stmt.rs-0285 */             }
/* FP:stmt.rs-0286 */         }
/* FP:stmt.rs-0287 */     }
/* FP:stmt.rs-0288 */ 
/* FP:stmt.rs-0289 */     fn recover_stmt_local_after_let(
/* FP:stmt.rs-0290 */         &mut self,
/* FP:stmt.rs-0291 */         lo: Span,
/* FP:stmt.rs-0292 */         attrs: AttrWrapper,
/* FP:stmt.rs-0293 */         subdiagnostic: fn(Span) -> errors::InvalidVariableDeclarationSub,
/* FP:stmt.rs-0294 */         force_collect: ForceCollect,
/* FP:stmt.rs-0295 */     ) -> PResult<'a, Stmt> {
/* FP:stmt.rs-0296 */         let stmt = self.collect_tokens(None, attrs, force_collect, |this, attrs| {
/* FP:stmt.rs-0297 */             let local = this.parse_local(None, attrs)?;
/* FP:stmt.rs-0298 */             // FIXME - maybe capture semicolon in recovery?
/* FP:stmt.rs-0299 */             Ok((
/* FP:stmt.rs-0300 */                 this.mk_stmt(lo.to(this.prev_token.span), StmtKind::Let(local)),
/* FP:stmt.rs-0301 */                 Trailing::No,
/* FP:stmt.rs-0302 */                 UsePreAttrPos::No,
/* FP:stmt.rs-0303 */             ))
/* FP:stmt.rs-0304 */         })?;
/* FP:stmt.rs-0305 */         self.dcx()
/* FP:stmt.rs-0306 */             .emit_err(errors::InvalidVariableDeclaration { span: lo, sub: subdiagnostic(lo) });
/* FP:stmt.rs-0307 */         Ok(stmt)
/* FP:stmt.rs-0308 */     }
/* FP:stmt.rs-0309 */ 
/* FP:stmt.rs-0310 */     /// Parses a local variable declaration.
/* FP:stmt.rs-0311 */     fn parse_local(&mut self, super_: Option<Span>, attrs: AttrVec) -> PResult<'a, Box<Local>> {
/* FP:stmt.rs-0312 */         let lo = super_.unwrap_or(self.prev_token.span);
/* FP:stmt.rs-0313 */ 
/* FP:stmt.rs-0314 */         if self.token.is_keyword(kw::Const) && self.look_ahead(1, |t| t.is_ident()) {
/* FP:stmt.rs-0315 */             self.dcx().emit_err(errors::ConstLetMutuallyExclusive { span: lo.to(self.token.span) });
/* FP:stmt.rs-0316 */             self.bump();
/* FP:stmt.rs-0317 */         }
/* FP:stmt.rs-0318 */ 
/* FP:stmt.rs-0319 */         let (pat, colon) =
/* FP:stmt.rs-0320 */             self.parse_pat_before_ty(None, RecoverComma::Yes, PatternLocation::LetBinding)?;
/* FP:stmt.rs-0321 */ 
/* FP:stmt.rs-0322 */         let (err, ty, colon_sp) = if colon {
/* FP:stmt.rs-0323 */             // Save the state of the parser before parsing type normally, in case there is a `:`
/* FP:stmt.rs-0324 */             // instead of an `=` typo.
/* FP:stmt.rs-0325 */             let parser_snapshot_before_type = self.clone();
/* FP:stmt.rs-0326 */             let colon_sp = self.prev_token.span;
/* FP:stmt.rs-0327 */             match self.parse_ty() {
/* FP:stmt.rs-0328 */                 Ok(ty) => (None, Some(ty), Some(colon_sp)),
/* FP:stmt.rs-0329 */                 Err(mut err) => {
/* FP:stmt.rs-0330 */                     err.span_label(
/* FP:stmt.rs-0331 */                         colon_sp,
/* FP:stmt.rs-0332 */                         format!(
/* FP:stmt.rs-0333 */                             "while parsing the type for {}",
/* FP:stmt.rs-0334 */                             pat.descr()
/* FP:stmt.rs-0335 */                                 .map_or_else(|| "the binding".to_string(), |n| format!("`{n}`"))
/* FP:stmt.rs-0336 */                         ),
/* FP:stmt.rs-0337 */                     );
/* FP:stmt.rs-0338 */                     // we use noexpect here because we don't actually expect Eq to be here
/* FP:stmt.rs-0339 */                     // but we are still checking for it in order to be able to handle it if
/* FP:stmt.rs-0340 */                     // it is there
/* FP:stmt.rs-0341 */                     let err = if self.check_noexpect(&token::Eq) {
/* FP:stmt.rs-0342 */                         err.emit();
/* FP:stmt.rs-0343 */                         None
/* FP:stmt.rs-0344 */                     } else {
/* FP:stmt.rs-0345 */                         // Rewind to before attempting to parse the type and continue parsing.
/* FP:stmt.rs-0346 */                         let parser_snapshot_after_type =
/* FP:stmt.rs-0347 */                             mem::replace(self, parser_snapshot_before_type);
/* FP:stmt.rs-0348 */                         Some((parser_snapshot_after_type, colon_sp, err))
/* FP:stmt.rs-0349 */                     };
/* FP:stmt.rs-0350 */                     (err, None, Some(colon_sp))
/* FP:stmt.rs-0351 */                 }
/* FP:stmt.rs-0352 */             }
/* FP:stmt.rs-0353 */         } else {
/* FP:stmt.rs-0354 */             (None, None, None)
/* FP:stmt.rs-0355 */         };
/* FP:stmt.rs-0356 */         let init = match (self.parse_initializer(err.is_some()), err) {
/* FP:stmt.rs-0357 */             (Ok(init), None) => {
/* FP:stmt.rs-0358 */                 // init parsed, ty parsed
/* FP:stmt.rs-0359 */                 init
/* FP:stmt.rs-0360 */             }
/* FP:stmt.rs-0361 */             (Ok(init), Some((_, colon_sp, mut err))) => {
/* FP:stmt.rs-0362 */                 // init parsed, ty error
/* FP:stmt.rs-0363 */                 // Could parse the type as if it were the initializer, it is likely there was a
/* FP:stmt.rs-0364 */                 // typo in the code: `:` instead of `=`. Add suggestion and emit the error.
/* FP:stmt.rs-0365 */                 err.span_suggestion_short(
/* FP:stmt.rs-0366 */                     colon_sp,
/* FP:stmt.rs-0367 */                     "use `=` if you meant to assign",
/* FP:stmt.rs-0368 */                     " =",
/* FP:stmt.rs-0369 */                     Applicability::MachineApplicable,
/* FP:stmt.rs-0370 */                 );
/* FP:stmt.rs-0371 */                 err.emit();
/* FP:stmt.rs-0372 */                 // As this was parsed successfully, continue as if the code has been fixed for the
/* FP:stmt.rs-0373 */                 // rest of the file. It will still fail due to the emitted error, but we avoid
/* FP:stmt.rs-0374 */                 // extra noise.
/* FP:stmt.rs-0375 */                 init
/* FP:stmt.rs-0376 */             }
/* FP:stmt.rs-0377 */             (Err(init_err), Some((snapshot, _, ty_err))) => {
/* FP:stmt.rs-0378 */                 // init error, ty error
/* FP:stmt.rs-0379 */                 init_err.cancel();
/* FP:stmt.rs-0380 */                 // Couldn't parse the type nor the initializer, only raise the type error and
/* FP:stmt.rs-0381 */                 // return to the parser state before parsing the type as the initializer.
/* FP:stmt.rs-0382 */                 // let x: <parse_error>;
/* FP:stmt.rs-0383 */                 *self = snapshot;
/* FP:stmt.rs-0384 */                 return Err(ty_err);
/* FP:stmt.rs-0385 */             }
/* FP:stmt.rs-0386 */             (Err(err), None) => {
/* FP:stmt.rs-0387 */                 // init error, ty parsed
/* FP:stmt.rs-0388 */                 // Couldn't parse the initializer and we're not attempting to recover a failed
/* FP:stmt.rs-0389 */                 // parse of the type, return the error.
/* FP:stmt.rs-0390 */                 return Err(err);
/* FP:stmt.rs-0391 */             }
/* FP:stmt.rs-0392 */         };
/* FP:stmt.rs-0393 */         let kind = match init {
/* FP:stmt.rs-0394 */             None => LocalKind::Decl,
/* FP:stmt.rs-0395 */             Some(init) => {
/* FP:stmt.rs-0396 */                 if self.eat_keyword(exp!(Else)) {
/* FP:stmt.rs-0397 */                     if self.token.is_keyword(kw::If) {
/* FP:stmt.rs-0398 */                         // `let...else if`. Emit the same error that `parse_block()` would,
/* FP:stmt.rs-0399 */                         // but explicitly point out that this pattern is not allowed.
/* FP:stmt.rs-0400 */                         let msg = "conditional `else if` is not supported for `let...else`";
/* FP:stmt.rs-0401 */                         return Err(self.error_block_no_opening_brace_msg(Cow::from(msg)));
/* FP:stmt.rs-0402 */                     }
/* FP:stmt.rs-0403 */                     let els = self.parse_block()?;
/* FP:stmt.rs-0404 */                     self.check_let_else_init_bool_expr(&init);
/* FP:stmt.rs-0405 */                     self.check_let_else_init_trailing_brace(&init);
/* FP:stmt.rs-0406 */                     LocalKind::InitElse(init, els)
/* FP:stmt.rs-0407 */                 } else {
/* FP:stmt.rs-0408 */                     LocalKind::Init(init)
/* FP:stmt.rs-0409 */                 }
/* FP:stmt.rs-0410 */             }
/* FP:stmt.rs-0411 */         };
/* FP:stmt.rs-0412 */         let hi = if self.token == token::Semi { self.token.span } else { self.prev_token.span };
/* FP:stmt.rs-0413 */         Ok(Box::new(ast::Local {
/* FP:stmt.rs-0414 */             super_,
/* FP:stmt.rs-0415 */             ty,
/* FP:stmt.rs-0416 */             pat,
/* FP:stmt.rs-0417 */             kind,
/* FP:stmt.rs-0418 */             id: DUMMY_NODE_ID,
/* FP:stmt.rs-0419 */             span: lo.to(hi),
/* FP:stmt.rs-0420 */             colon_sp,
/* FP:stmt.rs-0421 */             attrs,
/* FP:stmt.rs-0422 */             tokens: None,
/* FP:stmt.rs-0423 */         }))
/* FP:stmt.rs-0424 */     }
/* FP:stmt.rs-0425 */ 
/* FP:stmt.rs-0426 */     fn check_let_else_init_bool_expr(&self, init: &ast::Expr) {
/* FP:stmt.rs-0427 */         if let ast::ExprKind::Binary(op, ..) = init.kind {
/* FP:stmt.rs-0428 */             if op.node.is_lazy() {
/* FP:stmt.rs-0429 */                 self.dcx().emit_err(errors::InvalidExpressionInLetElse {
/* FP:stmt.rs-0430 */                     span: init.span,
/* FP:stmt.rs-0431 */                     operator: op.node.as_str(),
/* FP:stmt.rs-0432 */                     sugg: errors::WrapInParentheses::Expression {
/* FP:stmt.rs-0433 */                         left: init.span.shrink_to_lo(),
/* FP:stmt.rs-0434 */                         right: init.span.shrink_to_hi(),
/* FP:stmt.rs-0435 */                     },
/* FP:stmt.rs-0436 */                 });
/* FP:stmt.rs-0437 */             }
/* FP:stmt.rs-0438 */         }
/* FP:stmt.rs-0439 */     }
/* FP:stmt.rs-0440 */ 
/* FP:stmt.rs-0441 */     fn check_let_else_init_trailing_brace(&self, init: &ast::Expr) {
/* FP:stmt.rs-0442 */         if let Some(trailing) = classify::expr_trailing_brace(init) {
/* FP:stmt.rs-0443 */             let (span, sugg) = match trailing {
/* FP:stmt.rs-0444 */                 TrailingBrace::MacCall(mac) => (
/* FP:stmt.rs-0445 */                     mac.span(),
/* FP:stmt.rs-0446 */                     errors::WrapInParentheses::MacroArgs {
/* FP:stmt.rs-0447 */                         left: mac.args.dspan.open,
/* FP:stmt.rs-0448 */                         right: mac.args.dspan.close,
/* FP:stmt.rs-0449 */                     },
/* FP:stmt.rs-0450 */                 ),
/* FP:stmt.rs-0451 */                 TrailingBrace::Expr(expr) => (
/* FP:stmt.rs-0452 */                     expr.span,
/* FP:stmt.rs-0453 */                     errors::WrapInParentheses::Expression {
/* FP:stmt.rs-0454 */                         left: expr.span.shrink_to_lo(),
/* FP:stmt.rs-0455 */                         right: expr.span.shrink_to_hi(),
/* FP:stmt.rs-0456 */                     },
/* FP:stmt.rs-0457 */                 ),
/* FP:stmt.rs-0458 */             };
/* FP:stmt.rs-0459 */             self.dcx().emit_err(errors::InvalidCurlyInLetElse {
/* FP:stmt.rs-0460 */                 span: span.with_lo(span.hi() - BytePos(1)),
/* FP:stmt.rs-0461 */                 sugg,
/* FP:stmt.rs-0462 */             });
/* FP:stmt.rs-0463 */         }
/* FP:stmt.rs-0464 */     }
/* FP:stmt.rs-0465 */ 
/* FP:stmt.rs-0466 */     /// Parses the RHS of a local variable declaration (e.g., `= 14;`).
/* FP:stmt.rs-0467 */     fn parse_initializer(&mut self, eq_optional: bool) -> PResult<'a, Option<Box<Expr>>> {
/* FP:stmt.rs-0468 */         let eq_consumed = match self.token.kind {
/* FP:stmt.rs-0469 */             token::PlusEq
/* FP:stmt.rs-0470 */             | token::MinusEq
/* FP:stmt.rs-0471 */             | token::StarEq
/* FP:stmt.rs-0472 */             | token::SlashEq
/* FP:stmt.rs-0473 */             | token::PercentEq
/* FP:stmt.rs-0474 */             | token::CaretEq
/* FP:stmt.rs-0475 */             | token::AndEq
/* FP:stmt.rs-0476 */             | token::OrEq
/* FP:stmt.rs-0477 */             | token::ShlEq
/* FP:stmt.rs-0478 */             | token::ShrEq => {
/* FP:stmt.rs-0479 */                 // Recover `let x <op>= 1` as `let x = 1` We must not use `+ BytePos(1)` here
/* FP:stmt.rs-0480 */                 // because `<op>` can be a multi-byte lookalike that was recovered, e.g. `➖=` (the
/* FP:stmt.rs-0481 */                 // `➖` is a U+2796 Heavy Minus Sign Unicode Character) that was recovered as a
/* FP:stmt.rs-0482 */                 // `-=`.
/* FP:stmt.rs-0483 */                 let extra_op_span = self.psess.source_map().start_point(self.token.span);
/* FP:stmt.rs-0484 */                 self.dcx().emit_err(errors::CompoundAssignmentExpressionInLet {
/* FP:stmt.rs-0485 */                     span: self.token.span,
/* FP:stmt.rs-0486 */                     suggestion: extra_op_span,
/* FP:stmt.rs-0487 */                 });
/* FP:stmt.rs-0488 */                 self.bump();
/* FP:stmt.rs-0489 */                 true
/* FP:stmt.rs-0490 */             }
/* FP:stmt.rs-0491 */             _ => self.eat(exp!(Eq)),
/* FP:stmt.rs-0492 */         };
/* FP:stmt.rs-0493 */ 
/* FP:stmt.rs-0494 */         Ok(if eq_consumed || eq_optional { Some(self.parse_expr()?) } else { None })
/* FP:stmt.rs-0495 */     }
/* FP:stmt.rs-0496 */ 
/* FP:stmt.rs-0497 */     /// Parses a block. No inner attributes are allowed.
/* FP:stmt.rs-0498 */     pub fn parse_block(&mut self) -> PResult<'a, Box<Block>> {
/* FP:stmt.rs-0499 */         let (attrs, block) = self.parse_inner_attrs_and_block(None)?;
/* FP:stmt.rs-0500 */         if let [.., last] = &*attrs {
/* FP:stmt.rs-0501 */             let suggest_to_outer = match &last.kind {
/* FP:stmt.rs-0502 */                 ast::AttrKind::Normal(attr) => attr.item.is_valid_for_outer_style(),
/* FP:stmt.rs-0503 */                 _ => false,
/* FP:stmt.rs-0504 */             };
/* FP:stmt.rs-0505 */             self.error_on_forbidden_inner_attr(
/* FP:stmt.rs-0506 */                 last.span,
/* FP:stmt.rs-0507 */                 super::attr::InnerAttrPolicy::Forbidden(Some(
/* FP:stmt.rs-0508 */                     InnerAttrForbiddenReason::InCodeBlock,
/* FP:stmt.rs-0509 */                 )),
/* FP:stmt.rs-0510 */                 suggest_to_outer,
/* FP:stmt.rs-0511 */             );
/* FP:stmt.rs-0512 */         }
/* FP:stmt.rs-0513 */         Ok(block)
/* FP:stmt.rs-0514 */     }
/* FP:stmt.rs-0515 */ 
/* FP:stmt.rs-0516 */     fn error_block_no_opening_brace_msg(&mut self, msg: Cow<'static, str>) -> Diag<'a> {
/* FP:stmt.rs-0517 */         let prev = self.prev_token.span;
/* FP:stmt.rs-0518 */         let sp = self.token.span;
/* FP:stmt.rs-0519 */         let mut err = self.dcx().struct_span_err(sp, msg);
/* FP:stmt.rs-0520 */         self.label_expected_raw_ref(&mut err);
/* FP:stmt.rs-0521 */ 
/* FP:stmt.rs-0522 */         let do_not_suggest_help = self.token.is_keyword(kw::In)
/* FP:stmt.rs-0523 */             || self.token == token::Colon
/* FP:stmt.rs-0524 */             || self.prev_token.is_keyword(kw::Raw);
/* FP:stmt.rs-0525 */ 
/* FP:stmt.rs-0526 */         // Check to see if the user has written something like
/* FP:stmt.rs-0527 */         //
/* FP:stmt.rs-0528 */         //    if (cond)
/* FP:stmt.rs-0529 */         //      bar;
/* FP:stmt.rs-0530 */         //
/* FP:stmt.rs-0531 */         // which is valid in other languages, but not Rust.
/* FP:stmt.rs-0532 */         match self.parse_stmt_without_recovery(false, ForceCollect::No, false) {
/* FP:stmt.rs-0533 */             // If the next token is an open brace, e.g., we have:
/* FP:stmt.rs-0534 */             //
/* FP:stmt.rs-0535 */             //     if expr other_expr {
/* FP:stmt.rs-0536 */             //        ^    ^          ^- lookahead(1) is a brace
/* FP:stmt.rs-0537 */             //        |    |- current token is not "else"
/* FP:stmt.rs-0538 */             //        |- (statement we just parsed)
/* FP:stmt.rs-0539 */             //
/* FP:stmt.rs-0540 */             // the place-inside-a-block suggestion would be more likely wrong than right.
/* FP:stmt.rs-0541 */             //
/* FP:stmt.rs-0542 */             // FIXME(compiler-errors): this should probably parse an arbitrary expr and not
/* FP:stmt.rs-0543 */             // just lookahead one token, so we can see if there's a brace after _that_,
/* FP:stmt.rs-0544 */             // since we want to protect against:
/* FP:stmt.rs-0545 */             //     `if 1 1 + 1 {` being suggested as  `if { 1 } 1 + 1 {`
/* FP:stmt.rs-0546 */             //                                            +   +
/* FP:stmt.rs-0547 */             Ok(Some(_))
/* FP:stmt.rs-0548 */                 if (!self.token.is_keyword(kw::Else)
/* FP:stmt.rs-0549 */                     && self.look_ahead(1, |t| t == &token::OpenBrace))
/* FP:stmt.rs-0550 */                     || do_not_suggest_help => {}
/* FP:stmt.rs-0551 */             // Do not suggest `if foo println!("") {;}` (as would be seen in test for #46836).
/* FP:stmt.rs-0552 */             Ok(Some(Stmt { kind: StmtKind::Empty, .. })) => {}
/* FP:stmt.rs-0553 */             Ok(Some(stmt)) => {
/* FP:stmt.rs-0554 */                 let stmt_own_line = self.psess.source_map().is_line_before_span_empty(sp);
/* FP:stmt.rs-0555 */                 let stmt_span = if stmt_own_line && self.eat(exp!(Semi)) {
/* FP:stmt.rs-0556 */                     // Expand the span to include the semicolon.
/* FP:stmt.rs-0557 */                     stmt.span.with_hi(self.prev_token.span.hi())
/* FP:stmt.rs-0558 */                 } else {
/* FP:stmt.rs-0559 */                     stmt.span
/* FP:stmt.rs-0560 */                 };
/* FP:stmt.rs-0561 */                 self.suggest_fixes_misparsed_for_loop_head(
/* FP:stmt.rs-0562 */                     &mut err,
/* FP:stmt.rs-0563 */                     prev.between(sp),
/* FP:stmt.rs-0564 */                     stmt_span,
/* FP:stmt.rs-0565 */                     &stmt.kind,
/* FP:stmt.rs-0566 */                 );
/* FP:stmt.rs-0567 */             }
/* FP:stmt.rs-0568 */             Err(e) => {
/* FP:stmt.rs-0569 */                 e.delay_as_bug();
/* FP:stmt.rs-0570 */             }
/* FP:stmt.rs-0571 */             _ => {}
/* FP:stmt.rs-0572 */         }
/* FP:stmt.rs-0573 */         err.span_label(sp, "expected `{`");
/* FP:stmt.rs-0574 */         err
/* FP:stmt.rs-0575 */     }
/* FP:stmt.rs-0576 */ 
/* FP:stmt.rs-0577 */     fn suggest_fixes_misparsed_for_loop_head(
/* FP:stmt.rs-0578 */         &self,
/* FP:stmt.rs-0579 */         e: &mut Diag<'_>,
/* FP:stmt.rs-0580 */         between: Span,
/* FP:stmt.rs-0581 */         stmt_span: Span,
/* FP:stmt.rs-0582 */         stmt_kind: &StmtKind,
/* FP:stmt.rs-0583 */     ) {
/* FP:stmt.rs-0584 */         match (&self.token.kind, &stmt_kind) {
/* FP:stmt.rs-0585 */             (token::OpenBrace, StmtKind::Expr(expr)) if let ExprKind::Call(..) = expr.kind => {
/* FP:stmt.rs-0586 */                 // for _ in x y() {}
/* FP:stmt.rs-0587 */                 e.span_suggestion_verbose(
/* FP:stmt.rs-0588 */                     between,
/* FP:stmt.rs-0589 */                     "you might have meant to write a method call",
/* FP:stmt.rs-0590 */                     ".".to_string(),
/* FP:stmt.rs-0591 */                     Applicability::MaybeIncorrect,
/* FP:stmt.rs-0592 */                 );
/* FP:stmt.rs-0593 */             }
/* FP:stmt.rs-0594 */             (token::OpenBrace, StmtKind::Expr(expr)) if let ExprKind::Field(..) = expr.kind => {
/* FP:stmt.rs-0595 */                 // for _ in x y.z {}
/* FP:stmt.rs-0596 */                 e.span_suggestion_verbose(
/* FP:stmt.rs-0597 */                     between,
/* FP:stmt.rs-0598 */                     "you might have meant to write a field access",
/* FP:stmt.rs-0599 */                     ".".to_string(),
/* FP:stmt.rs-0600 */                     Applicability::MaybeIncorrect,
/* FP:stmt.rs-0601 */                 );
/* FP:stmt.rs-0602 */             }
/* FP:stmt.rs-0603 */             (token::CloseBrace, StmtKind::Expr(expr))
/* FP:stmt.rs-0604 */                 if let ExprKind::Struct(expr) = &expr.kind
/* FP:stmt.rs-0605 */                     && let None = expr.qself
/* FP:stmt.rs-0606 */                     && expr.path.segments.len() == 1 =>
/* FP:stmt.rs-0607 */             {
/* FP:stmt.rs-0608 */                 // This is specific to "mistyped `if` condition followed by empty body"
/* FP:stmt.rs-0609 */                 //
/* FP:stmt.rs-0610 */                 // for _ in x y {}
/* FP:stmt.rs-0611 */                 e.span_suggestion_verbose(
/* FP:stmt.rs-0612 */                     between,
/* FP:stmt.rs-0613 */                     "you might have meant to write a field access",
/* FP:stmt.rs-0614 */                     ".".to_string(),
/* FP:stmt.rs-0615 */                     Applicability::MaybeIncorrect,
/* FP:stmt.rs-0616 */                 );
/* FP:stmt.rs-0617 */             }
/* FP:stmt.rs-0618 */             (token::OpenBrace, StmtKind::Expr(expr))
/* FP:stmt.rs-0619 */                 if let ExprKind::Lit(lit) = expr.kind
/* FP:stmt.rs-0620 */                     && let None = lit.suffix
/* FP:stmt.rs-0621 */                     && let token::LitKind::Integer | token::LitKind::Float = lit.kind =>
/* FP:stmt.rs-0622 */             {
/* FP:stmt.rs-0623 */                 // for _ in x 0 {}
/* FP:stmt.rs-0624 */                 // for _ in x 0.0 {}
/* FP:stmt.rs-0625 */                 e.span_suggestion_verbose(
/* FP:stmt.rs-0626 */                     between,
/* FP:stmt.rs-0627 */                     format!("you might have meant to write a field access"),
/* FP:stmt.rs-0628 */                     ".".to_string(),
/* FP:stmt.rs-0629 */                     Applicability::MaybeIncorrect,
/* FP:stmt.rs-0630 */                 );
/* FP:stmt.rs-0631 */             }
/* FP:stmt.rs-0632 */             (token::OpenBrace, StmtKind::Expr(expr))
/* FP:stmt.rs-0633 */                 if let ExprKind::Loop(..)
/* FP:stmt.rs-0634 */                 | ExprKind::If(..)
/* FP:stmt.rs-0635 */                 | ExprKind::While(..)
/* FP:stmt.rs-0636 */                 | ExprKind::Match(..)
/* FP:stmt.rs-0637 */                 | ExprKind::ForLoop { .. }
/* FP:stmt.rs-0638 */                 | ExprKind::TryBlock(..)
/* FP:stmt.rs-0639 */                 | ExprKind::Ret(..)
/* FP:stmt.rs-0640 */                 | ExprKind::Closure(..)
/* FP:stmt.rs-0641 */                 | ExprKind::Struct(..)
/* FP:stmt.rs-0642 */                 | ExprKind::Try(..) = expr.kind =>
/* FP:stmt.rs-0643 */             {
/* FP:stmt.rs-0644 */                 // These are more likely to have been meant as a block body.
/* FP:stmt.rs-0645 */                 e.multipart_suggestion(
/* FP:stmt.rs-0646 */                     "you might have meant to write this as part of a block",
/* FP:stmt.rs-0647 */                     vec![
/* FP:stmt.rs-0648 */                         (stmt_span.shrink_to_lo(), "{ ".to_string()),
/* FP:stmt.rs-0649 */                         (stmt_span.shrink_to_hi(), " }".to_string()),
/* FP:stmt.rs-0650 */                     ],
/* FP:stmt.rs-0651 */                     // Speculative; has been misleading in the past (#46836).
/* FP:stmt.rs-0652 */                     Applicability::MaybeIncorrect,
/* FP:stmt.rs-0653 */                 );
/* FP:stmt.rs-0654 */             }
/* FP:stmt.rs-0655 */             (token::OpenBrace, _) => {}
/* FP:stmt.rs-0656 */             (_, _) => {
/* FP:stmt.rs-0657 */                 e.multipart_suggestion(
/* FP:stmt.rs-0658 */                     "you might have meant to write this as part of a block",
/* FP:stmt.rs-0659 */                     vec![
/* FP:stmt.rs-0660 */                         (stmt_span.shrink_to_lo(), "{ ".to_string()),
/* FP:stmt.rs-0661 */                         (stmt_span.shrink_to_hi(), " }".to_string()),
/* FP:stmt.rs-0662 */                     ],
/* FP:stmt.rs-0663 */                     // Speculative; has been misleading in the past (#46836).
/* FP:stmt.rs-0664 */                     Applicability::MaybeIncorrect,
/* FP:stmt.rs-0665 */                 );
/* FP:stmt.rs-0666 */             }
/* FP:stmt.rs-0667 */         }
/* FP:stmt.rs-0668 */     }
/* FP:stmt.rs-0669 */ 
/* FP:stmt.rs-0670 */     fn error_block_no_opening_brace<T>(&mut self) -> PResult<'a, T> {
/* FP:stmt.rs-0671 */         let tok = super::token_descr(&self.token);
/* FP:stmt.rs-0672 */         let msg = format!("expected `{{`, found {tok}");
/* FP:stmt.rs-0673 */         Err(self.error_block_no_opening_brace_msg(Cow::from(msg)))
/* FP:stmt.rs-0674 */     }
/* FP:stmt.rs-0675 */ 
/* FP:stmt.rs-0676 */     /// Parses a block. Inner attributes are allowed, block labels are not.
/* FP:stmt.rs-0677 */     ///
/* FP:stmt.rs-0678 */     /// If `loop_header` is `Some` and an unexpected block label is encountered,
/* FP:stmt.rs-0679 */     /// it is suggested to be moved just before `loop_header`, else it is suggested to be removed.
/* FP:stmt.rs-0680 */     pub(super) fn parse_inner_attrs_and_block(
/* FP:stmt.rs-0681 */         &mut self,
/* FP:stmt.rs-0682 */         loop_header: Option<Span>,
/* FP:stmt.rs-0683 */     ) -> PResult<'a, (AttrVec, Box<Block>)> {
/* FP:stmt.rs-0684 */         self.parse_block_common(self.token.span, BlockCheckMode::Default, loop_header)
/* FP:stmt.rs-0685 */     }
/* FP:stmt.rs-0686 */ 
/* FP:stmt.rs-0687 */     /// Parses a block. Inner attributes are allowed, block labels are not.
/* FP:stmt.rs-0688 */     ///
/* FP:stmt.rs-0689 */     /// If `loop_header` is `Some` and an unexpected block label is encountered,
/* FP:stmt.rs-0690 */     /// it is suggested to be moved just before `loop_header`, else it is suggested to be removed.
/* FP:stmt.rs-0691 */     pub(super) fn parse_block_common(
/* FP:stmt.rs-0692 */         &mut self,
/* FP:stmt.rs-0693 */         lo: Span,
/* FP:stmt.rs-0694 */         blk_mode: BlockCheckMode,
/* FP:stmt.rs-0695 */         loop_header: Option<Span>,
/* FP:stmt.rs-0696 */     ) -> PResult<'a, (AttrVec, Box<Block>)> {
/* FP:stmt.rs-0697 */         if let Some(block) = self.eat_metavar_seq(MetaVarKind::Block, |this| this.parse_block()) {
/* FP:stmt.rs-0698 */             return Ok((AttrVec::new(), block));
/* FP:stmt.rs-0699 */         }
/* FP:stmt.rs-0700 */ 
/* FP:stmt.rs-0701 */         let maybe_ident = self.prev_token;
/* FP:stmt.rs-0702 */         self.maybe_recover_unexpected_block_label(loop_header);
/* FP:stmt.rs-0703 */         if !self.eat(exp!(OpenBrace)) {
/* FP:stmt.rs-0704 */             return self.error_block_no_opening_brace();
/* FP:stmt.rs-0705 */         }
/* FP:stmt.rs-0706 */ 
/* FP:stmt.rs-0707 */         let attrs = self.parse_inner_attributes()?;
/* FP:stmt.rs-0708 */         let tail = match self.maybe_suggest_struct_literal(lo, blk_mode, maybe_ident) {
/* FP:stmt.rs-0709 */             Some(tail) => tail?,
/* FP:stmt.rs-0710 */             None => self.parse_block_tail(lo, blk_mode, AttemptLocalParseRecovery::Yes)?,
/* FP:stmt.rs-0711 */         };
/* FP:stmt.rs-0712 */         Ok((attrs, tail))
/* FP:stmt.rs-0713 */     }
/* FP:stmt.rs-0714 */ 
/* FP:stmt.rs-0715 */     /// Parses the rest of a block expression or function body.
/* FP:stmt.rs-0716 */     /// Precondition: already parsed the '{'.
/* FP:stmt.rs-0717 */     pub fn parse_block_tail(
/* FP:stmt.rs-0718 */         &mut self,
/* FP:stmt.rs-0719 */         lo: Span,
/* FP:stmt.rs-0720 */         s: BlockCheckMode,
/* FP:stmt.rs-0721 */         recover: AttemptLocalParseRecovery,
/* FP:stmt.rs-0722 */     ) -> PResult<'a, Box<Block>> {
/* FP:stmt.rs-0723 */         let mut stmts = ThinVec::new();
/* FP:stmt.rs-0724 */         let mut snapshot = None;
/* FP:stmt.rs-0725 */         while !self.eat(exp!(CloseBrace)) {
/* FP:stmt.rs-0726 */             if self.token == token::Eof {
/* FP:stmt.rs-0727 */                 break;
/* FP:stmt.rs-0728 */             }
/* FP:stmt.rs-0729 */             if self.is_vcs_conflict_marker(&TokenKind::Shl, &TokenKind::Lt) {
/* FP:stmt.rs-0730 */                 // Account for `<<<<<<<` diff markers. We can't proactively error here because
/* FP:stmt.rs-0731 */                 // that can be a valid path start, so we snapshot and reparse only we've
/* FP:stmt.rs-0732 */                 // encountered another parse error.
/* FP:stmt.rs-0733 */                 snapshot = Some(self.create_snapshot_for_diagnostic());
/* FP:stmt.rs-0734 */             }
/* FP:stmt.rs-0735 */             let stmt = match self.parse_full_stmt(recover) {
/* FP:stmt.rs-0736 */                 Err(mut err) if recover.yes() => {
/* FP:stmt.rs-0737 */                     if let Some(ref mut snapshot) = snapshot {
/* FP:stmt.rs-0738 */                         snapshot.recover_vcs_conflict_marker();
/* FP:stmt.rs-0739 */                     }
/* FP:stmt.rs-0740 */                     if self.token == token::Colon {
/* FP:stmt.rs-0741 */                         // if a previous and next token of the current one is
/* FP:stmt.rs-0742 */                         // integer literal (e.g. `1:42`), it's likely a range
/* FP:stmt.rs-0743 */                         // expression for Pythonistas and we can suggest so.
/* FP:stmt.rs-0744 */                         if self.prev_token.is_integer_lit()
/* FP:stmt.rs-0745 */                             && self.may_recover()
/* FP:stmt.rs-0746 */                             && self.look_ahead(1, |token| token.is_integer_lit())
/* FP:stmt.rs-0747 */                         {
/* FP:stmt.rs-0748 */                             // FIXME(hkmatsumoto): Might be better to trigger
/* FP:stmt.rs-0749 */                             // this only when parsing an index expression.
/* FP:stmt.rs-0750 */                             err.span_suggestion_verbose(
/* FP:stmt.rs-0751 */                                 self.token.span,
/* FP:stmt.rs-0752 */                                 "you might have meant a range expression",
/* FP:stmt.rs-0753 */                                 "..",
/* FP:stmt.rs-0754 */                                 Applicability::MaybeIncorrect,
/* FP:stmt.rs-0755 */                             );
/* FP:stmt.rs-0756 */                         } else {
/* FP:stmt.rs-0757 */                             // if next token is following a colon, it's likely a path
/* FP:stmt.rs-0758 */                             // and we can suggest a path separator
/* FP:stmt.rs-0759 */                             self.bump();
/* FP:stmt.rs-0760 */                             if self.token.span.lo() == self.prev_token.span.hi() {
/* FP:stmt.rs-0761 */                                 err.span_suggestion_verbose(
/* FP:stmt.rs-0762 */                                     self.prev_token.span,
/* FP:stmt.rs-0763 */                                     "maybe write a path separator here",
/* FP:stmt.rs-0764 */                                     "::",
/* FP:stmt.rs-0765 */                                     Applicability::MaybeIncorrect,
/* FP:stmt.rs-0766 */                                 );
/* FP:stmt.rs-0767 */                             }
/* FP:stmt.rs-0768 */                         }
/* FP:stmt.rs-0769 */                     }
/* FP:stmt.rs-0770 */ 
/* FP:stmt.rs-0771 */                     let guar = err.emit();
/* FP:stmt.rs-0772 */                     self.recover_stmt_(SemiColonMode::Ignore, BlockMode::Ignore);
/* FP:stmt.rs-0773 */                     Some(self.mk_stmt_err(self.token.span, guar))
/* FP:stmt.rs-0774 */                 }
/* FP:stmt.rs-0775 */                 Ok(stmt) => stmt,
/* FP:stmt.rs-0776 */                 Err(err) => return Err(err),
/* FP:stmt.rs-0777 */             };
/* FP:stmt.rs-0778 */             if let Some(stmt) = stmt {
/* FP:stmt.rs-0779 */                 stmts.push(stmt);
/* FP:stmt.rs-0780 */             } else {
/* FP:stmt.rs-0781 */                 // Found only `;` or `}`.
/* FP:stmt.rs-0782 */                 continue;
/* FP:stmt.rs-0783 */             };
/* FP:stmt.rs-0784 */         }
/* FP:stmt.rs-0785 */         Ok(self.mk_block(stmts, s, lo.to(self.prev_token.span)))
/* FP:stmt.rs-0786 */     }
/* FP:stmt.rs-0787 */ 
/* FP:stmt.rs-0788 */     fn recover_missing_let_else(&mut self, err: &mut Diag<'_>, pat: &ast::Pat, stmt_span: Span) {
/* FP:stmt.rs-0789 */         if self.token.kind != token::OpenBrace {
/* FP:stmt.rs-0790 */             return;
/* FP:stmt.rs-0791 */         }
/* FP:stmt.rs-0792 */         match pat.kind {
/* FP:stmt.rs-0793 */             ast::PatKind::Ident(..) | ast::PatKind::Missing | ast::PatKind::Wild => {
/* FP:stmt.rs-0794 */                 // Not if let or let else
/* FP:stmt.rs-0795 */                 return;
/* FP:stmt.rs-0796 */             }
/* FP:stmt.rs-0797 */             _ => {}
/* FP:stmt.rs-0798 */         }
/* FP:stmt.rs-0799 */         let snapshot = self.create_snapshot_for_diagnostic();
/* FP:stmt.rs-0800 */         let block_span = self.token.span;
/* FP:stmt.rs-0801 */         let (if_let, let_else) = match self.parse_block() {
/* FP:stmt.rs-0802 */             Ok(block) => {
/* FP:stmt.rs-0803 */                 let mut idents = vec![];
/* FP:stmt.rs-0804 */                 pat.walk(&mut |pat: &ast::Pat| {
/* FP:stmt.rs-0805 */                     if let ast::PatKind::Ident(_, ident, _) = pat.kind {
/* FP:stmt.rs-0806 */                         idents.push(ident);
/* FP:stmt.rs-0807 */                     }
/* FP:stmt.rs-0808 */                     true
/* FP:stmt.rs-0809 */                 });
/* FP:stmt.rs-0810 */ 
/* FP:stmt.rs-0811 */                 struct IdentFinder {
/* FP:stmt.rs-0812 */                     idents: Vec<Ident>,
/* FP:stmt.rs-0813 */                     /// If a block references one of the bindings introduced by the let pattern,
/* FP:stmt.rs-0814 */                     /// we likely meant to use `if let`.
/* FP:stmt.rs-0815 */                     /// This is pre-expansion, so if we encounter
/* FP:stmt.rs-0816 */                     /// `let Some(x) = foo() { println!("{x}") }` we won't find it.
/* FP:stmt.rs-0817 */                     references_ident: bool = false,
/* FP:stmt.rs-0818 */                     /// If a block has a `return`, then we know with high certainty that it was
/* FP:stmt.rs-0819 */                     /// meant to be let-else.
/* FP:stmt.rs-0820 */                     has_return: bool = false,
/* FP:stmt.rs-0821 */                 }
/* FP:stmt.rs-0822 */ 
/* FP:stmt.rs-0823 */                 impl<'a> Visitor<'a> for IdentFinder {
/* FP:stmt.rs-0824 */                     fn visit_ident(&mut self, ident: &Ident) {
/* FP:stmt.rs-0825 */                         for i in &self.idents {
/* FP:stmt.rs-0826 */                             if ident.name == i.name {
/* FP:stmt.rs-0827 */                                 self.references_ident = true;
/* FP:stmt.rs-0828 */                             }
/* FP:stmt.rs-0829 */                         }
/* FP:stmt.rs-0830 */                     }
/* FP:stmt.rs-0831 */                     fn visit_expr(&mut self, node: &'a Expr) {
/* FP:stmt.rs-0832 */                         if let ExprKind::Ret(..) = node.kind {
/* FP:stmt.rs-0833 */                             self.has_return = true;
/* FP:stmt.rs-0834 */                         }
/* FP:stmt.rs-0835 */                         walk_expr(self, node);
/* FP:stmt.rs-0836 */                     }
/* FP:stmt.rs-0837 */                 }
/* FP:stmt.rs-0838 */ 
/* FP:stmt.rs-0839 */                 // Collect all bindings in pattern and see if they appear in the block. Likely meant
/* FP:stmt.rs-0840 */                 // to write `if let`. See if the block has a return. Likely meant to write
/* FP:stmt.rs-0841 */                 // `let else`.
/* FP:stmt.rs-0842 */                 let mut visitor = IdentFinder { idents, .. };
/* FP:stmt.rs-0843 */                 visitor.visit_block(&block);
/* FP:stmt.rs-0844 */ 
/* FP:stmt.rs-0845 */                 (visitor.references_ident, visitor.has_return)
/* FP:stmt.rs-0846 */             }
/* FP:stmt.rs-0847 */             Err(e) => {
/* FP:stmt.rs-0848 */                 e.cancel();
/* FP:stmt.rs-0849 */                 self.restore_snapshot(snapshot);
/* FP:stmt.rs-0850 */                 (false, false)
/* FP:stmt.rs-0851 */             }
/* FP:stmt.rs-0852 */         };
/* FP:stmt.rs-0853 */ 
/* FP:stmt.rs-0854 */         let mut alternatively = "";
/* FP:stmt.rs-0855 */         if if_let || !let_else {
/* FP:stmt.rs-0856 */             alternatively = "alternatively, ";
/* FP:stmt.rs-0857 */             err.span_suggestion_verbose(
/* FP:stmt.rs-0858 */                 stmt_span.shrink_to_lo(),
/* FP:stmt.rs-0859 */                 "you might have meant to use `if let`",
/* FP:stmt.rs-0860 */                 "if ".to_string(),
/* FP:stmt.rs-0861 */                 if if_let {
/* FP:stmt.rs-0862 */                     Applicability::MachineApplicable
/* FP:stmt.rs-0863 */                 } else {
/* FP:stmt.rs-0864 */                     Applicability::MaybeIncorrect
/* FP:stmt.rs-0865 */                 },
/* FP:stmt.rs-0866 */             );
/* FP:stmt.rs-0867 */         }
/* FP:stmt.rs-0868 */         if let_else || !if_let {
/* FP:stmt.rs-0869 */             err.span_suggestion_verbose(
/* FP:stmt.rs-0870 */                 block_span.shrink_to_lo(),
/* FP:stmt.rs-0871 */                 format!("{alternatively}you might have meant to use `let else`"),
/* FP:stmt.rs-0872 */                 "else ".to_string(),
/* FP:stmt.rs-0873 */                 if let_else {
/* FP:stmt.rs-0874 */                     Applicability::MachineApplicable
/* FP:stmt.rs-0875 */                 } else {
/* FP:stmt.rs-0876 */                     Applicability::MaybeIncorrect
/* FP:stmt.rs-0877 */                 },
/* FP:stmt.rs-0878 */             );
/* FP:stmt.rs-0879 */         }
/* FP:stmt.rs-0880 */     }
/* FP:stmt.rs-0881 */ 
/* FP:stmt.rs-0882 */     fn recover_missing_dot(&mut self, err: &mut Diag<'_>) {
/* FP:stmt.rs-0883 */         let Some((ident, _)) = self.token.ident() else {
/* FP:stmt.rs-0884 */             return;
/* FP:stmt.rs-0885 */         };
/* FP:stmt.rs-0886 */         if let Some(c) = ident.name.as_str().chars().next()
/* FP:stmt.rs-0887 */             && c.is_uppercase()
/* FP:stmt.rs-0888 */         {
/* FP:stmt.rs-0889 */             return;
/* FP:stmt.rs-0890 */         }
/* FP:stmt.rs-0891 */         if self.token.is_reserved_ident() && !self.token.is_ident_named(kw::Await) {
/* FP:stmt.rs-0892 */             return;
/* FP:stmt.rs-0893 */         }
/* FP:stmt.rs-0894 */         if self.prev_token.is_reserved_ident() && self.prev_token.is_ident_named(kw::Await) {
/* FP:stmt.rs-0895 */             // Likely `foo.await bar`
/* FP:stmt.rs-0896 */         } else if self.prev_token.is_non_reserved_ident() {
/* FP:stmt.rs-0897 */             // Likely `foo bar`
/* FP:stmt.rs-0898 */         } else if self.prev_token.kind == token::Question {
/* FP:stmt.rs-0899 */             // `foo? bar`
/* FP:stmt.rs-0900 */         } else if self.prev_token.kind == token::CloseParen {
/* FP:stmt.rs-0901 */             // `foo() bar`
/* FP:stmt.rs-0902 */         } else {
/* FP:stmt.rs-0903 */             return;
/* FP:stmt.rs-0904 */         }
/* FP:stmt.rs-0905 */         if self.token.span == self.prev_token.span {
/* FP:stmt.rs-0906 */             // Account for syntax errors in proc-macros.
/* FP:stmt.rs-0907 */             return;
/* FP:stmt.rs-0908 */         }
/* FP:stmt.rs-0909 */         if self.look_ahead(1, |t| [token::Semi, token::Question, token::Dot].contains(&t.kind)) {
/* FP:stmt.rs-0910 */             err.span_suggestion_verbose(
/* FP:stmt.rs-0911 */                 self.prev_token.span.between(self.token.span),
/* FP:stmt.rs-0912 */                 "you might have meant to write a field access",
/* FP:stmt.rs-0913 */                 ".".to_string(),
/* FP:stmt.rs-0914 */                 Applicability::MaybeIncorrect,
/* FP:stmt.rs-0915 */             );
/* FP:stmt.rs-0916 */         }
/* FP:stmt.rs-0917 */         if self.look_ahead(1, |t| t.kind == token::OpenParen) {
/* FP:stmt.rs-0918 */             err.span_suggestion_verbose(
/* FP:stmt.rs-0919 */                 self.prev_token.span.between(self.token.span),
/* FP:stmt.rs-0920 */                 "you might have meant to write a method call",
/* FP:stmt.rs-0921 */                 ".".to_string(),
/* FP:stmt.rs-0922 */                 Applicability::MaybeIncorrect,
/* FP:stmt.rs-0923 */             );
/* FP:stmt.rs-0924 */         }
/* FP:stmt.rs-0925 */     }
/* FP:stmt.rs-0926 */ 
/* FP:stmt.rs-0927 */     /// Parses a statement, including the trailing semicolon.
/* FP:stmt.rs-0928 */     pub fn parse_full_stmt(
/* FP:stmt.rs-0929 */         &mut self,
/* FP:stmt.rs-0930 */         recover: AttemptLocalParseRecovery,
/* FP:stmt.rs-0931 */     ) -> PResult<'a, Option<Stmt>> {
/* FP:stmt.rs-0932 */         // Skip looking for a trailing semicolon when we have a metavar seq.
/* FP:stmt.rs-0933 */         if let Some(stmt) = self.eat_metavar_seq(MetaVarKind::Stmt, |this| {
/* FP:stmt.rs-0934 */             // Why pass `true` for `force_full_expr`? Statement expressions are less expressive
/* FP:stmt.rs-0935 */             // than "full" expressions, due to the `STMT_EXPR` restriction, and sometimes need
/* FP:stmt.rs-0936 */             // parentheses. E.g. the "full" expression `match paren_around_match {} | true` when
/* FP:stmt.rs-0937 */             // used in statement context must be written `(match paren_around_match {} | true)`.
/* FP:stmt.rs-0938 */             // However, if the expression we are parsing in this statement context was pasted by a
/* FP:stmt.rs-0939 */             // declarative macro, it may have come from a "full" expression context, and lack
/* FP:stmt.rs-0940 */             // these parentheses. So we lift the `STMT_EXPR` restriction to ensure the statement
/* FP:stmt.rs-0941 */             // will reparse successfully.
/* FP:stmt.rs-0942 */             this.parse_stmt_without_recovery(false, ForceCollect::No, true)
/* FP:stmt.rs-0943 */         }) {
/* FP:stmt.rs-0944 */             let stmt = stmt.expect("an actual statement");
/* FP:stmt.rs-0945 */             return Ok(Some(stmt));
/* FP:stmt.rs-0946 */         }
/* FP:stmt.rs-0947 */ 
/* FP:stmt.rs-0948 */         let Some(mut stmt) = self.parse_stmt_without_recovery(true, ForceCollect::No, false)?
/* FP:stmt.rs-0949 */         else {
/* FP:stmt.rs-0950 */             return Ok(None);
/* FP:stmt.rs-0951 */         };
/* FP:stmt.rs-0952 */ 
/* FP:stmt.rs-0953 */         let mut eat_semi = true;
/* FP:stmt.rs-0954 */         let mut add_semi_to_stmt = false;
/* FP:stmt.rs-0955 */ 
/* FP:stmt.rs-0956 */         match &mut stmt.kind {
/* FP:stmt.rs-0957 */             // Expression without semicolon.
/* FP:stmt.rs-0958 */             StmtKind::Expr(expr)
/* FP:stmt.rs-0959 */                 if classify::expr_requires_semi_to_be_stmt(expr)
/* FP:stmt.rs-0960 */                     && !expr.attrs.is_empty()
/* FP:stmt.rs-0961 */                     && !matches!(self.token.kind, token::Eof | token::Semi | token::CloseBrace) =>
/* FP:stmt.rs-0962 */             {
/* FP:stmt.rs-0963 */                 // The user has written `#[attr] expr` which is unsupported. (#106020)
/* FP:stmt.rs-0964 */                 let guar = self.attr_on_non_tail_expr(&expr);
/* FP:stmt.rs-0965 */                 // We already emitted an error, so don't emit another type error
/* FP:stmt.rs-0966 */                 let sp = expr.span.to(self.prev_token.span);
/* FP:stmt.rs-0967 */                 *expr = self.mk_expr_err(sp, guar);
/* FP:stmt.rs-0968 */             }
/* FP:stmt.rs-0969 */ 
/* FP:stmt.rs-0970 */             // Expression without semicolon.
/* FP:stmt.rs-0971 */             StmtKind::Expr(expr)
/* FP:stmt.rs-0972 */                 if self.token != token::Eof && classify::expr_requires_semi_to_be_stmt(expr) =>
/* FP:stmt.rs-0973 */             {
/* FP:stmt.rs-0974 */                 // Just check for errors and recover; do not eat semicolon yet.
/* FP:stmt.rs-0975 */ 
/* FP:stmt.rs-0976 */                 let expect_result =
/* FP:stmt.rs-0977 */                     if let Err(e) = self.maybe_recover_from_ternary_operator(Some(expr.span)) {
/* FP:stmt.rs-0978 */                         Err(e)
/* FP:stmt.rs-0979 */                     } else {
/* FP:stmt.rs-0980 */                         self.expect_one_of(&[], &[exp!(Semi), exp!(CloseBrace)])
/* FP:stmt.rs-0981 */                     };
/* FP:stmt.rs-0982 */ 
/* FP:stmt.rs-0983 */                 // Try to both emit a better diagnostic, and avoid further errors by replacing
/* FP:stmt.rs-0984 */                 // the `expr` with `ExprKind::Err`.
/* FP:stmt.rs-0985 */                 let replace_with_err = 'break_recover: {
/* FP:stmt.rs-0986 */                     match expect_result {
/* FP:stmt.rs-0987 */                         Ok(Recovered::No) => None,
/* FP:stmt.rs-0988 */                         Ok(Recovered::Yes(guar)) => {
/* FP:stmt.rs-0989 */                             // Skip type error to avoid extra errors.
/* FP:stmt.rs-0990 */                             Some(guar)
/* FP:stmt.rs-0991 */                         }
/* FP:stmt.rs-0992 */                         Err(e) => {
/* FP:stmt.rs-0993 */                             if self.recover_colon_as_semi() {
/* FP:stmt.rs-0994 */                                 // recover_colon_as_semi has already emitted a nicer error.
/* FP:stmt.rs-0995 */                                 e.delay_as_bug();
/* FP:stmt.rs-0996 */                                 add_semi_to_stmt = true;
/* FP:stmt.rs-0997 */                                 eat_semi = false;
/* FP:stmt.rs-0998 */ 
/* FP:stmt.rs-0999 */                                 break 'break_recover None;
/* FP:stmt.rs-1000 */                             }
/* FP:stmt.rs-1001 */ 
/* FP:stmt.rs-1002 */                             match &expr.kind {
/* FP:stmt.rs-1003 */                                 ExprKind::Path(None, ast::Path { segments, .. })
/* FP:stmt.rs-1004 */                                     if let [segment] = segments.as_slice() =>
/* FP:stmt.rs-1005 */                                 {
/* FP:stmt.rs-1006 */                                     if self.token == token::Colon
/* FP:stmt.rs-1007 */                                         && self.look_ahead(1, |token| {
/* FP:stmt.rs-1008 */                                             token.is_metavar_block()
/* FP:stmt.rs-1009 */                                                 || matches!(
/* FP:stmt.rs-1010 */                                                     token.kind,
/* FP:stmt.rs-1011 */                                                     token::Ident(
/* FP:stmt.rs-1012 */                                                         kw::For | kw::Loop | kw::While,
/* FP:stmt.rs-1013 */                                                         token::IdentIsRaw::No
/* FP:stmt.rs-1014 */                                                     ) | token::OpenBrace
/* FP:stmt.rs-1015 */                                                 )
/* FP:stmt.rs-1016 */                                         })
/* FP:stmt.rs-1017 */                                     {
/* FP:stmt.rs-1018 */                                         let snapshot = self.create_snapshot_for_diagnostic();
/* FP:stmt.rs-1019 */                                         let label = Label {
/* FP:stmt.rs-1020 */                                             ident: Ident::from_str_and_span(
/* FP:stmt.rs-1021 */                                                 &format!("'{}", segment.ident),
/* FP:stmt.rs-1022 */                                                 segment.ident.span,
/* FP:stmt.rs-1023 */                                             ),
/* FP:stmt.rs-1024 */                                         };
/* FP:stmt.rs-1025 */                                         match self.parse_expr_labeled(label, false) {
/* FP:stmt.rs-1026 */                                             Ok(labeled_expr) => {
/* FP:stmt.rs-1027 */                                                 e.cancel();
/* FP:stmt.rs-1028 */                                                 self.dcx().emit_err(MalformedLoopLabel {
/* FP:stmt.rs-1029 */                                                     span: label.ident.span,
/* FP:stmt.rs-1030 */                                                     suggestion: label.ident.span.shrink_to_lo(),
/* FP:stmt.rs-1031 */                                                 });
/* FP:stmt.rs-1032 */                                                 *expr = labeled_expr;
/* FP:stmt.rs-1033 */                                                 break 'break_recover None;
/* FP:stmt.rs-1034 */                                             }
/* FP:stmt.rs-1035 */                                             Err(err) => {
/* FP:stmt.rs-1036 */                                                 err.cancel();
/* FP:stmt.rs-1037 */                                                 self.restore_snapshot(snapshot);
/* FP:stmt.rs-1038 */                                             }
/* FP:stmt.rs-1039 */                                         }
/* FP:stmt.rs-1040 */                                     }
/* FP:stmt.rs-1041 */                                 }
/* FP:stmt.rs-1042 */                                 _ => {}
/* FP:stmt.rs-1043 */                             }
/* FP:stmt.rs-1044 */ 
/* FP:stmt.rs-1045 */                             let res =
/* FP:stmt.rs-1046 */                                 self.check_mistyped_turbofish_with_multiple_type_params(e, expr);
/* FP:stmt.rs-1047 */ 
/* FP:stmt.rs-1048 */                             Some(if recover.no() {
/* FP:stmt.rs-1049 */                                 res?
/* FP:stmt.rs-1050 */                             } else {
/* FP:stmt.rs-1051 */                                 res.unwrap_or_else(|mut e| {
/* FP:stmt.rs-1052 */                                     self.recover_missing_dot(&mut e);
/* FP:stmt.rs-1053 */                                     let guar = e.emit();
/* FP:stmt.rs-1054 */                                     self.recover_stmt();
/* FP:stmt.rs-1055 */                                     guar
/* FP:stmt.rs-1056 */                                 })
/* FP:stmt.rs-1057 */                             })
/* FP:stmt.rs-1058 */                         }
/* FP:stmt.rs-1059 */                     }
/* FP:stmt.rs-1060 */                 };
/* FP:stmt.rs-1061 */ 
/* FP:stmt.rs-1062 */                 if let Some(guar) = replace_with_err {
/* FP:stmt.rs-1063 */                     // We already emitted an error, so don't emit another type error
/* FP:stmt.rs-1064 */                     let sp = expr.span.to(self.prev_token.span);
/* FP:stmt.rs-1065 */                     *expr = self.mk_expr_err(sp, guar);
/* FP:stmt.rs-1066 */                 }
/* FP:stmt.rs-1067 */             }
/* FP:stmt.rs-1068 */             StmtKind::Expr(_) | StmtKind::MacCall(_) => {}
/* FP:stmt.rs-1069 */             StmtKind::Let(local) if let Err(mut e) = self.expect_semi() => {
/* FP:stmt.rs-1070 */                 // We might be at the `,` in `let x = foo<bar, baz>;`. Try to recover.
/* FP:stmt.rs-1071 */                 match &mut local.kind {
/* FP:stmt.rs-1072 */                     LocalKind::Init(expr) | LocalKind::InitElse(expr, _) => {
/* FP:stmt.rs-1073 */                         self.check_mistyped_turbofish_with_multiple_type_params(e, expr).map_err(
/* FP:stmt.rs-1074 */                             |mut e| {
/* FP:stmt.rs-1075 */                                 self.recover_missing_dot(&mut e);
/* FP:stmt.rs-1076 */                                 self.recover_missing_let_else(&mut e, &local.pat, stmt.span);
/* FP:stmt.rs-1077 */                                 e
/* FP:stmt.rs-1078 */                             },
/* FP:stmt.rs-1079 */                         )?;
/* FP:stmt.rs-1080 */                         // We found `foo<bar, baz>`, have we fully recovered?
/* FP:stmt.rs-1081 */                         self.expect_semi()?;
/* FP:stmt.rs-1082 */                     }
/* FP:stmt.rs-1083 */                     LocalKind::Decl => {
/* FP:stmt.rs-1084 */                         if let Some(colon_sp) = local.colon_sp {
/* FP:stmt.rs-1085 */                             e.span_label(
/* FP:stmt.rs-1086 */                                 colon_sp,
/* FP:stmt.rs-1087 */                                 format!(
/* FP:stmt.rs-1088 */                                     "while parsing the type for {}",
/* FP:stmt.rs-1089 */                                     local.pat.descr().map_or_else(
/* FP:stmt.rs-1090 */                                         || "the binding".to_string(),
/* FP:stmt.rs-1091 */                                         |n| format!("`{n}`")
/* FP:stmt.rs-1092 */                                     )
/* FP:stmt.rs-1093 */                                 ),
/* FP:stmt.rs-1094 */                             );
/* FP:stmt.rs-1095 */                             let suggest_eq = if self.token == token::Dot
/* FP:stmt.rs-1096 */                                 && let _ = self.bump()
/* FP:stmt.rs-1097 */                                 && let mut snapshot = self.create_snapshot_for_diagnostic()
/* FP:stmt.rs-1098 */                                 && let Ok(_) = snapshot
/* FP:stmt.rs-1099 */                                     .parse_dot_suffix_expr(
/* FP:stmt.rs-1100 */                                         colon_sp,
/* FP:stmt.rs-1101 */                                         self.mk_expr_err(
/* FP:stmt.rs-1102 */                                             colon_sp,
/* FP:stmt.rs-1103 */                                             self.dcx()
/* FP:stmt.rs-1104 */                                                 .delayed_bug("error during `:` -> `=` recovery"),
/* FP:stmt.rs-1105 */                                         ),
/* FP:stmt.rs-1106 */                                     )
/* FP:stmt.rs-1107 */                                     .map_err(Diag::cancel)
/* FP:stmt.rs-1108 */                             {
/* FP:stmt.rs-1109 */                                 true
/* FP:stmt.rs-1110 */                             } else if let Some(op) = self.check_assoc_op()
/* FP:stmt.rs-1111 */                                 && op.node.can_continue_expr_unambiguously()
/* FP:stmt.rs-1112 */                             {
/* FP:stmt.rs-1113 */                                 true
/* FP:stmt.rs-1114 */                             } else {
/* FP:stmt.rs-1115 */                                 false
/* FP:stmt.rs-1116 */                             };
/* FP:stmt.rs-1117 */                             if suggest_eq {
/* FP:stmt.rs-1118 */                                 e.span_suggestion_short(
/* FP:stmt.rs-1119 */                                     colon_sp,
/* FP:stmt.rs-1120 */                                     "use `=` if you meant to assign",
/* FP:stmt.rs-1121 */                                     "=",
/* FP:stmt.rs-1122 */                                     Applicability::MaybeIncorrect,
/* FP:stmt.rs-1123 */                                 );
/* FP:stmt.rs-1124 */                             }
/* FP:stmt.rs-1125 */                         }
/* FP:stmt.rs-1126 */                         return Err(e);
/* FP:stmt.rs-1127 */                     }
/* FP:stmt.rs-1128 */                 }
/* FP:stmt.rs-1129 */                 eat_semi = false;
/* FP:stmt.rs-1130 */             }
/* FP:stmt.rs-1131 */             StmtKind::Empty | StmtKind::Item(_) | StmtKind::Let(_) | StmtKind::Semi(_) => {
/* FP:stmt.rs-1132 */                 eat_semi = false
/* FP:stmt.rs-1133 */             }
/* FP:stmt.rs-1134 */         }
/* FP:stmt.rs-1135 */ 
/* FP:stmt.rs-1136 */         if add_semi_to_stmt || (eat_semi && self.eat(exp!(Semi))) {
/* FP:stmt.rs-1137 */             stmt = stmt.add_trailing_semicolon();
/* FP:stmt.rs-1138 */         }
/* FP:stmt.rs-1139 */ 
/* FP:stmt.rs-1140 */         stmt.span = stmt.span.to(self.prev_token.span);
/* FP:stmt.rs-1141 */         Ok(Some(stmt))
/* FP:stmt.rs-1142 */     }
/* FP:stmt.rs-1143 */ 
/* FP:stmt.rs-1144 */     pub(super) fn mk_block(
/* FP:stmt.rs-1145 */         &self,
/* FP:stmt.rs-1146 */         stmts: ThinVec<Stmt>,
/* FP:stmt.rs-1147 */         rules: BlockCheckMode,
/* FP:stmt.rs-1148 */         span: Span,
/* FP:stmt.rs-1149 */     ) -> Box<Block> {
/* FP:stmt.rs-1150 */         Box::new(Block { stmts, id: DUMMY_NODE_ID, rules, span, tokens: None })
/* FP:stmt.rs-1151 */     }
/* FP:stmt.rs-1152 */ 
/* FP:stmt.rs-1153 */     pub(super) fn mk_stmt(&self, span: Span, kind: StmtKind) -> Stmt {
/* FP:stmt.rs-1154 */         Stmt { id: DUMMY_NODE_ID, kind, span }
/* FP:stmt.rs-1155 */     }
/* FP:stmt.rs-1156 */ 
/* FP:stmt.rs-1157 */     pub(super) fn mk_stmt_err(&self, span: Span, guar: ErrorGuaranteed) -> Stmt {
/* FP:stmt.rs-1158 */         self.mk_stmt(span, StmtKind::Expr(self.mk_expr_err(span, guar)))
/* FP:stmt.rs-1159 */     }
/* FP:stmt.rs-1160 */ 
/* FP:stmt.rs-1161 */     pub(super) fn mk_block_err(&self, span: Span, guar: ErrorGuaranteed) -> Box<Block> {
/* FP:stmt.rs-1162 */         self.mk_block(thin_vec![self.mk_stmt_err(span, guar)], BlockCheckMode::Default, span)
/* FP:stmt.rs-1163 */     }
/* FP:stmt.rs-1164 */ }