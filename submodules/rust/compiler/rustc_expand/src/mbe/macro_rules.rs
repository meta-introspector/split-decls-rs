mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: collections :: hash_map :: Entry ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use std :: { mem , slice } ;}
mkuse!{use ast :: token :: IdentIsRaw ;}
mkuse!{use rustc_ast :: token :: NtPatKind :: * ;}
mkuse!{use rustc_ast :: token :: TokenKind :: * ;}
mkuse!{use rustc_ast :: token :: { self , Delimiter , NonterminalKind , Token , TokenKind } ;}
mkuse!{use rustc_ast :: tokenstream :: { self , DelimSpan , TokenStream } ;}
mkuse!{use rustc_ast :: { self as ast , DUMMY_NODE_ID , NodeId } ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxIndexMap } ;}
mkuse!{use rustc_errors :: { Applicability , Diag , ErrorGuaranteed , MultiSpan } ;}
mkuse!{use rustc_feature :: Features ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: MacroKinds ;}
mkuse!{use rustc_hir :: find_attr ;}
mkuse!{use rustc_lint_defs :: BuiltinLintDiag ;}
mkuse!{use rustc_lint_defs :: builtin :: { RUST_2021_INCOMPATIBLE_OR_PATTERNS , SEMICOLON_IN_EXPRESSIONS_FROM_MACROS , } ;}
mkuse!{use rustc_parse :: exp ;}
mkuse!{use rustc_parse :: parser :: { Parser , Recovery } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_session :: parse :: { ParseSess , feature_err } ;}
mkuse!{use rustc_span :: edition :: Edition ;}
mkuse!{use rustc_span :: hygiene :: Transparency ;}
mkuse!{use rustc_span :: { Ident , Span , Symbol , kw , sym } ;}
mkuse!{use tracing :: { debug , instrument , trace , trace_span } ;}
mkuse!{use super :: diagnostics :: { FailedMacro , failed_to_match_macro } ;}
mkuse!{use super :: macro_parser :: { NamedMatches , NamedParseResult } ;}
mkuse!{use super :: { SequenceRepetition , diagnostics } ;}
mkuse!{use crate :: base :: { AttrProcMacro , DummyResult , ExpandResult , ExtCtxt , MacResult , MacroExpanderResult , SyntaxExtension , SyntaxExtensionKind , TTMacroExpander , } ;}
mkuse!{use crate :: errors ;}
mkuse!{use crate :: expand :: { AstFragment , AstFragmentKind , ensure_complete_parse , parse_ast_fragment } ;}
mkuse!{use crate :: mbe :: macro_check :: check_meta_variables ;}
mkuse!{use crate :: mbe :: macro_parser :: { Error , ErrorReported , Failure , MatcherLoc , Success , TtParser } ;}
mkuse!{use crate :: mbe :: quoted :: { RulePart , parse_one_tt } ;}
mkuse!{use crate :: mbe :: transcribe :: transcribe ;}
mkuse!{use crate :: mbe :: { self , KleeneOp } ;}
mkitem!{mkstruct!{pub (crate) struct ParserAnyMacro < 'a > { parser : Parser < 'a > , # [doc = " Span of the expansion site of the macro this parser is for"] site_span : Span , # [doc = " The ident of the macro we're parsing"] macro_ident : Ident , lint_node_id : NodeId , is_trailing_mac : bool , arm_span : Span , # [doc = " Whether or not this macro is defined in the current crate"] is_local : bool , }}}
mkitem!{mkimpl!{impl < 'a > ParserAnyMacro < 'a > { pub (crate) fn make (mut self : Box < ParserAnyMacro < 'a > > , kind : AstFragmentKind) -> AstFragment { let ParserAnyMacro { site_span , macro_ident , ref mut parser , lint_node_id , arm_span , is_trailing_mac , is_local , } = * self ; let snapshot = & mut parser . create_snapshot_for_diagnostic () ; let fragment = match parse_ast_fragment (parser , kind) { Ok (f) => f , Err (err) => { let guar = diagnostics :: emit_frag_parse_err (err , parser , snapshot , site_span , arm_span , kind ,) ; return kind . dummy (site_span , guar) ; } } ; if kind == AstFragmentKind :: Expr && parser . token == token :: Semi { if is_local { parser . psess . buffer_lint (SEMICOLON_IN_EXPRESSIONS_FROM_MACROS , parser . token . span , lint_node_id , BuiltinLintDiag :: TrailingMacro (is_trailing_mac , macro_ident) ,) ; } parser . bump () ; } let path = ast :: Path :: from_ident (macro_ident . with_span_pos (site_span)) ; ensure_complete_parse (parser , & path , kind . name () , site_span) ; fragment } # [instrument (skip (cx , tts))] pub (crate) fn from_tts < 'cx > (cx : & 'cx mut ExtCtxt < 'a > , tts : TokenStream , site_span : Span , arm_span : Span , is_local : bool , macro_ident : Ident ,) -> Self { Self { parser : Parser :: new (& cx . sess . psess , tts , None) , site_span , macro_ident , lint_node_id : cx . current_expansion . lint_node_id , is_trailing_mac : cx . current_expansion . is_trailing_mac , arm_span , is_local , } } }}}
mkitem!{mkenum!{pub (super) enum MacroRule { # [doc = " A function-style rule, for use with `m!()`"] Func { lhs : Vec < MatcherLoc > , lhs_span : Span , rhs : mbe :: TokenTree } , # [doc = " An attr rule, for use with `#[m]`"] Attr { args : Vec < MatcherLoc > , args_span : Span , body : Vec < MatcherLoc > , body_span : Span , rhs : mbe :: TokenTree , } , # [doc = " A derive rule, for use with `#[m]`"] Derive { body : Vec < MatcherLoc > , body_span : Span , rhs : mbe :: TokenTree } , }}}
mkitem!{mkstruct!{pub struct MacroRulesMacroExpander { node_id : NodeId , name : Ident , span : Span , transparency : Transparency , kinds : MacroKinds , rules : Vec < MacroRule > , }}}
mkitem!{mkimpl!{impl MacroRulesMacroExpander { pub fn get_unused_rule (& self , rule_i : usize) -> Option < (& Ident , MultiSpan) > { let (span , rhs) = match self . rules [rule_i] { MacroRule :: Func { lhs_span , ref rhs , .. } => (MultiSpan :: from_span (lhs_span) , rhs) , MacroRule :: Attr { args_span , body_span , ref rhs , .. } => { (MultiSpan :: from_spans (vec ! [args_span , body_span]) , rhs) } MacroRule :: Derive { body_span , ref rhs , .. } => (MultiSpan :: from_span (body_span) , rhs) , } ; if has_compile_error_macro (rhs) { None } else { Some ((& self . name , span)) } } pub fn kinds (& self) -> MacroKinds { self . kinds } pub fn expand_derive (& self , cx : & mut ExtCtxt < '_ > , sp : Span , body : & TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { let Self { name , ref rules , node_id , .. } = * self ; let psess = & cx . sess . psess ; if cx . trace_macros () { let msg = format ! ("expanding `#[derive({name})] {}`" , pprust :: tts_to_string (body)) ; trace_macros_note (& mut cx . expansions , sp , msg) ; } match try_match_macro_derive (psess , name , body , rules , & mut NoopTracker) { Ok ((rule_index , rule , named_matches)) => { let MacroRule :: Derive { rhs , .. } = rule else { panic ! ("try_match_macro_derive returned non-derive rule") ; } ; let mbe :: TokenTree :: Delimited (rhs_span , _ , rhs) = rhs else { cx . dcx () . span_bug (sp , "malformed macro derive rhs") ; } ; let id = cx . current_expansion . id ; let tts = transcribe (psess , & named_matches , rhs , * rhs_span , self . transparency , id) . map_err (| e | e . emit ()) ? ; if cx . trace_macros () { let msg = format ! ("to `{}`" , pprust :: tts_to_string (& tts)) ; trace_macros_note (& mut cx . expansions , sp , msg) ; } if is_defined_in_current_crate (node_id) { cx . resolver . record_macro_rule_usage (node_id , rule_index) ; } Ok (tts) } Err (CanRetry :: No (guar)) => Err (guar) , Err (CanRetry :: Yes) => { let (_ , guar) = failed_to_match_macro (cx . psess () , sp , self . span , name , FailedMacro :: Derive , body , rules ,) ; cx . macro_error_and_trace_macros_diag () ; Err (guar) } } } }}}
mkitem!{mkimpl!{impl TTMacroExpander for MacroRulesMacroExpander { fn expand < 'cx > (& self , cx : & 'cx mut ExtCtxt < '_ > , sp : Span , input : TokenStream ,) -> MacroExpanderResult < 'cx > { ExpandResult :: Ready (expand_macro (cx , sp , self . span , self . node_id , self . name , self . transparency , input , & self . rules ,)) } }}}
mkitem!{mkimpl!{impl AttrProcMacro for MacroRulesMacroExpander { fn expand (& self , cx : & mut ExtCtxt < '_ > , sp : Span , args : TokenStream , body : TokenStream ,) -> Result < TokenStream , ErrorGuaranteed > { expand_macro_attr (cx , sp , self . span , self . node_id , self . name , self . transparency , args , body , & self . rules ,) } }}}
mkitem!{mkstruct!{struct DummyExpander (ErrorGuaranteed) ;}}
mkitem!{mkimpl!{impl TTMacroExpander for DummyExpander { fn expand < 'cx > (& self , _ : & 'cx mut ExtCtxt < '_ > , span : Span , _ : TokenStream ,) -> ExpandResult < Box < dyn MacResult + 'cx > , () > { ExpandResult :: Ready (DummyResult :: any (span , self . 0)) } }}}

macro_rules! trace_macros_note_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trace_macros_note in module {}", module_path!());
    };
}

mkfn!{
    trace_macros_note_introspect!();
    fn trace_macros_note (cx_expansions : & mut FxIndexMap < Span , Vec < String > > , sp : Span , message : String) { let sp = sp . macro_backtrace () . last () . map_or (sp , | trace | trace . call_site) ; cx_expansions . entry (sp) . or_default () . push (message) ; }
}
mkitem!{mktrait!{pub (super) trait Tracker < 'matcher > { # [doc = " The contents of `ParseResult::Failure`."] type Failure ; # [doc = " Arm failed to match. If the token is `token::Eof`, it indicates an unexpected"] # [doc = " end of macro invocation. Otherwise, it indicates that no rules expected the given token."] # [doc = " The usize is the approximate position of the token in the input token stream."] fn build_failure (tok : Token , position : u32 , msg : & 'static str) -> Self :: Failure ; # [doc = " This is called before trying to match next MatcherLoc on the current token."] fn before_match_loc (& mut self , _parser : & TtParser , _matcher : & 'matcher MatcherLoc) { } # [doc = " This is called after an arm has been parsed, either successfully or unsuccessfully. When"] # [doc = " this is called, `before_match_loc` was called at least once (with a `MatcherLoc::Eof`)."] fn after_arm (& mut self , _in_body : bool , _result : & NamedParseResult < Self :: Failure >) { } # [doc = " For tracing."] fn description () -> & 'static str ; fn recovery () -> Recovery { Recovery :: Forbidden } }}}
mkitem!{mkstruct!{# [doc = " A noop tracker that is used in the hot path of the expansion, has zero overhead thanks to"] # [doc = " monomorphization."] pub (super) struct NoopTracker ;}}
mkitem!{mkimpl!{impl < 'matcher > Tracker < 'matcher > for NoopTracker { type Failure = () ; fn build_failure (_tok : Token , _position : u32 , _msg : & 'static str) -> Self :: Failure { } fn description () -> & 'static str { "none" } }}}

macro_rules! expand_macro_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_macro in module {}", module_path!());
    };
}

mkfn!{
    expand_macro_introspect!();
    # [doc = " Expands the rules based macro defined by `rules` for a given input `arg`."] # [instrument (skip (cx , transparency , arg , rules))] fn expand_macro < 'cx > (cx : & 'cx mut ExtCtxt < '_ > , sp : Span , def_span : Span , node_id : NodeId , name : Ident , transparency : Transparency , arg : TokenStream , rules : & [MacroRule] ,) -> Box < dyn MacResult + 'cx > { let psess = & cx . sess . psess ; if cx . trace_macros () { let msg = format ! ("expanding `{}! {{ {} }}`" , name , pprust :: tts_to_string (& arg)) ; trace_macros_note (& mut cx . expansions , sp , msg) ; } let try_success_result = try_match_macro (psess , name , & arg , rules , & mut NoopTracker) ; match try_success_result { Ok ((rule_index , rule , named_matches)) => { let MacroRule :: Func { rhs , .. } = rule else { panic ! ("try_match_macro returned non-func rule") ; } ; let mbe :: TokenTree :: Delimited (rhs_span , _ , rhs) = rhs else { cx . dcx () . span_bug (sp , "malformed macro rhs") ; } ; let arm_span = rhs_span . entire () ; let id = cx . current_expansion . id ; let tts = match transcribe (psess , & named_matches , rhs , * rhs_span , transparency , id) { Ok (tts) => tts , Err (err) => { let guar = err . emit () ; return DummyResult :: any (arm_span , guar) ; } } ; if cx . trace_macros () { let msg = format ! ("to `{}`" , pprust :: tts_to_string (& tts)) ; trace_macros_note (& mut cx . expansions , sp , msg) ; } let is_local = is_defined_in_current_crate (node_id) ; if is_local { cx . resolver . record_macro_rule_usage (node_id , rule_index) ; } Box :: new (ParserAnyMacro :: from_tts (cx , tts , sp , arm_span , is_local , name)) } Err (CanRetry :: No (guar)) => { debug ! ("Will not retry matching as an error was emitted already") ; DummyResult :: any (sp , guar) } Err (CanRetry :: Yes) => { let (span , guar) = failed_to_match_macro (cx . psess () , sp , def_span , name , FailedMacro :: Func , & arg , rules ,) ; cx . macro_error_and_trace_macros_diag () ; DummyResult :: any (span , guar) } } }
}

macro_rules! expand_macro_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expand_macro_attr in module {}", module_path!());
    };
}

mkfn!{
    expand_macro_attr_introspect!();
    # [doc = " Expands the rules based macro defined by `rules` for a given attribute `args` and `body`."] # [instrument (skip (cx , transparency , args , body , rules))] fn expand_macro_attr (cx : & mut ExtCtxt < '_ > , sp : Span , def_span : Span , node_id : NodeId , name : Ident , transparency : Transparency , args : TokenStream , body : TokenStream , rules : & [MacroRule] ,) -> Result < TokenStream , ErrorGuaranteed > { let psess = & cx . sess . psess ; let is_local = node_id != DUMMY_NODE_ID ; if cx . trace_macros () { let msg = format ! ("expanding `#[{name}({})] {}`" , pprust :: tts_to_string (& args) , pprust :: tts_to_string (& body) ,) ; trace_macros_note (& mut cx . expansions , sp , msg) ; } match try_match_macro_attr (psess , name , & args , & body , rules , & mut NoopTracker) { Ok ((i , rule , named_matches)) => { let MacroRule :: Attr { rhs , .. } = rule else { panic ! ("try_macro_match_attr returned non-attr rule") ; } ; let mbe :: TokenTree :: Delimited (rhs_span , _ , rhs) = rhs else { cx . dcx () . span_bug (sp , "malformed macro rhs") ; } ; let id = cx . current_expansion . id ; let tts = transcribe (psess , & named_matches , rhs , * rhs_span , transparency , id) . map_err (| e | e . emit ()) ? ; if cx . trace_macros () { let msg = format ! ("to `{}`" , pprust :: tts_to_string (& tts)) ; trace_macros_note (& mut cx . expansions , sp , msg) ; } if is_local { cx . resolver . record_macro_rule_usage (node_id , i) ; } Ok (tts) } Err (CanRetry :: No (guar)) => Err (guar) , Err (CanRetry :: Yes) => { let (_ , guar) = failed_to_match_macro (cx . psess () , sp , def_span , name , FailedMacro :: Attr (& args) , & body , rules ,) ; cx . trace_macros_diag () ; Err (guar) } } }
}
mkitem!{mkenum!{pub (super) enum CanRetry { Yes , # [doc = " We are not allowed to retry macro expansion as a fatal error has been emitted already."] No (ErrorGuaranteed) , }}}

macro_rules! try_match_macro_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_match_macro in module {}", module_path!());
    };
}

mkfn!{
    try_match_macro_introspect!();
    # [doc = " Try expanding the macro. Returns the index of the successful arm and its named_matches if it was successful,"] # [doc = " and nothing if it failed. On failure, it's the callers job to use `track` accordingly to record all errors"] # [doc = " correctly."] # [instrument (level = "debug" , skip (psess , arg , rules , track) , fields (tracking = % T :: description ()))] pub (super) fn try_match_macro < 'matcher , T : Tracker < 'matcher > > (psess : & ParseSess , name : Ident , arg : & TokenStream , rules : & 'matcher [MacroRule] , track : & mut T ,) -> Result < (usize , & 'matcher MacroRule , NamedMatches) , CanRetry > { let parser = parser_from_cx (psess , arg . clone () , T :: recovery ()) ; let mut tt_parser = TtParser :: new (name) ; for (i , rule) in rules . iter () . enumerate () { let MacroRule :: Func { lhs , .. } = rule else { continue } ; let _tracing_span = trace_span ! ("Matching arm" , % i) ; let mut gated_spans_snapshot = mem :: take (& mut * psess . gated_spans . spans . borrow_mut ()) ; let result = tt_parser . parse_tt (& mut Cow :: Borrowed (& parser) , lhs , track) ; track . after_arm (true , & result) ; match result { Success (named_matches) => { debug ! ("Parsed arm successfully") ; psess . gated_spans . merge (gated_spans_snapshot) ; return Ok ((i , rule , named_matches)) ; } Failure (_) => { trace ! ("Failed to match arm, trying the next one") ; } Error (_ , _) => { debug ! ("Fatal error occurred during matching") ; return Err (CanRetry :: Yes) ; } ErrorReported (guarantee) => { debug ! ("Fatal error occurred and was reported during matching") ; return Err (CanRetry :: No (guarantee)) ; } } mem :: swap (& mut gated_spans_snapshot , & mut psess . gated_spans . spans . borrow_mut ()) ; } Err (CanRetry :: Yes) }
}

macro_rules! try_match_macro_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_match_macro_attr in module {}", module_path!());
    };
}

mkfn!{
    try_match_macro_attr_introspect!();
    # [doc = " Try expanding the macro attribute. Returns the index of the successful arm and its"] # [doc = " named_matches if it was successful, and nothing if it failed. On failure, it's the caller's job"] # [doc = " to use `track` accordingly to record all errors correctly."] # [instrument (level = "debug" , skip (psess , attr_args , attr_body , rules , track) , fields (tracking = % T :: description ()))] pub (super) fn try_match_macro_attr < 'matcher , T : Tracker < 'matcher > > (psess : & ParseSess , name : Ident , attr_args : & TokenStream , attr_body : & TokenStream , rules : & 'matcher [MacroRule] , track : & mut T ,) -> Result < (usize , & 'matcher MacroRule , NamedMatches) , CanRetry > { let args_parser = parser_from_cx (psess , attr_args . clone () , T :: recovery ()) ; let body_parser = parser_from_cx (psess , attr_body . clone () , T :: recovery ()) ; let mut tt_parser = TtParser :: new (name) ; for (i , rule) in rules . iter () . enumerate () { let MacroRule :: Attr { args , body , .. } = rule else { continue } ; let mut gated_spans_snapshot = mem :: take (& mut * psess . gated_spans . spans . borrow_mut ()) ; let result = tt_parser . parse_tt (& mut Cow :: Borrowed (& args_parser) , args , track) ; track . after_arm (false , & result) ; let mut named_matches = match result { Success (named_matches) => named_matches , Failure (_) => { mem :: swap (& mut gated_spans_snapshot , & mut psess . gated_spans . spans . borrow_mut ()) ; continue ; } Error (_ , _) => return Err (CanRetry :: Yes) , ErrorReported (guar) => return Err (CanRetry :: No (guar)) , } ; let result = tt_parser . parse_tt (& mut Cow :: Borrowed (& body_parser) , body , track) ; track . after_arm (true , & result) ; match result { Success (body_named_matches) => { psess . gated_spans . merge (gated_spans_snapshot) ; # [allow (rustc :: potential_query_instability)] named_matches . extend (body_named_matches) ; return Ok ((i , rule , named_matches)) ; } Failure (_) => { mem :: swap (& mut gated_spans_snapshot , & mut psess . gated_spans . spans . borrow_mut ()) } Error (_ , _) => return Err (CanRetry :: Yes) , ErrorReported (guar) => return Err (CanRetry :: No (guar)) , } } Err (CanRetry :: Yes) }
}

macro_rules! try_match_macro_derive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_match_macro_derive in module {}", module_path!());
    };
}

mkfn!{
    try_match_macro_derive_introspect!();
    # [doc = " Try expanding the macro derive. Returns the index of the successful arm and its"] # [doc = " named_matches if it was successful, and nothing if it failed. On failure, it's the caller's job"] # [doc = " to use `track` accordingly to record all errors correctly."] # [instrument (level = "debug" , skip (psess , body , rules , track) , fields (tracking = % T :: description ()))] pub (super) fn try_match_macro_derive < 'matcher , T : Tracker < 'matcher > > (psess : & ParseSess , name : Ident , body : & TokenStream , rules : & 'matcher [MacroRule] , track : & mut T ,) -> Result < (usize , & 'matcher MacroRule , NamedMatches) , CanRetry > { let body_parser = parser_from_cx (psess , body . clone () , T :: recovery ()) ; let mut tt_parser = TtParser :: new (name) ; for (i , rule) in rules . iter () . enumerate () { let MacroRule :: Derive { body , .. } = rule else { continue } ; let mut gated_spans_snapshot = mem :: take (& mut * psess . gated_spans . spans . borrow_mut ()) ; let result = tt_parser . parse_tt (& mut Cow :: Borrowed (& body_parser) , body , track) ; track . after_arm (true , & result) ; match result { Success (named_matches) => { psess . gated_spans . merge (gated_spans_snapshot) ; return Ok ((i , rule , named_matches)) ; } Failure (_) => { mem :: swap (& mut gated_spans_snapshot , & mut psess . gated_spans . spans . borrow_mut ()) } Error (_ , _) => return Err (CanRetry :: Yes) , ErrorReported (guar) => return Err (CanRetry :: No (guar)) , } } Err (CanRetry :: Yes) }
}

macro_rules! compile_declarative_macro_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compile_declarative_macro in module {}", module_path!());
    };
}

mkfn!{
    compile_declarative_macro_introspect!();
    # [doc = " Converts a macro item into a syntax extension."] pub fn compile_declarative_macro (sess : & Session , features : & Features , macro_def : & ast :: MacroDef , ident : Ident , attrs : & [hir :: Attribute] , span : Span , node_id : NodeId , edition : Edition ,) -> (SyntaxExtension , usize) { let mk_syn_ext = | kind | { let is_local = is_defined_in_current_crate (node_id) ; SyntaxExtension :: new (sess , kind , span , Vec :: new () , edition , ident . name , attrs , is_local) } ; let dummy_syn_ext = | guar | (mk_syn_ext (SyntaxExtensionKind :: LegacyBang (Arc :: new (DummyExpander (guar)))) , 0) ; let macro_rules = macro_def . macro_rules ; let exp_sep = if macro_rules { exp ! (Semi) } else { exp ! (Comma) } ; let body = macro_def . body . tokens . clone () ; let mut p = Parser :: new (& sess . psess , body , rustc_parse :: MACRO_ARGUMENTS) ; let mut guar = None ; let mut check_emission = | ret : Result < () , ErrorGuaranteed > | guar = guar . or (ret . err ()) ; let mut kinds = MacroKinds :: empty () ; let mut rules = Vec :: new () ; while p . token != token :: Eof { let (args , is_derive) = if p . eat_keyword_noexpect (sym :: attr) { kinds |= MacroKinds :: ATTR ; if ! features . macro_attr () { feature_err (sess , sym :: macro_attr , span , "`macro_rules!` attributes are unstable") . emit () ; } if let Some (guar) = check_no_eof (sess , & p , "expected macro attr args") { return dummy_syn_ext (guar) ; } let args = p . parse_token_tree () ; check_args_parens (sess , sym :: attr , & args) ; let args = parse_one_tt (args , RulePart :: Pattern , sess , node_id , features , edition) ; check_emission (check_lhs (sess , node_id , & args)) ; if let Some (guar) = check_no_eof (sess , & p , "expected macro attr body") { return dummy_syn_ext (guar) ; } (Some (args) , false) } else if p . eat_keyword_noexpect (sym :: derive) { kinds |= MacroKinds :: DERIVE ; let derive_keyword_span = p . prev_token . span ; if ! features . macro_derive () { feature_err (sess , sym :: macro_attr , span , "`macro_rules!` derives are unstable") . emit () ; } if let Some (guar) = check_no_eof (sess , & p , "expected `()` after `derive`") { return dummy_syn_ext (guar) ; } let args = p . parse_token_tree () ; check_args_parens (sess , sym :: derive , & args) ; let args_empty_result = check_args_empty (sess , & args) ; let args_not_empty = args_empty_result . is_err () ; check_emission (args_empty_result) ; if let Some (guar) = check_no_eof (sess , & p , "expected macro derive body") { return dummy_syn_ext (guar) ; } if p . token == token :: FatArrow { let mut err = sess . dcx () . struct_span_err (p . token . span , "expected macro derive body, got `=>`") ; if args_not_empty { err . span_label (derive_keyword_span , "need `()` after this `derive`") ; } return dummy_syn_ext (err . emit ()) ; } (None , true) } else { kinds |= MacroKinds :: BANG ; (None , false) } ; let lhs_tt = p . parse_token_tree () ; let lhs_tt = parse_one_tt (lhs_tt , RulePart :: Pattern , sess , node_id , features , edition) ; check_emission (check_lhs (sess , node_id , & lhs_tt)) ; if let Err (e) = p . expect (exp ! (FatArrow)) { return dummy_syn_ext (e . emit ()) ; } if let Some (guar) = check_no_eof (sess , & p , "expected right-hand side of macro rule") { return dummy_syn_ext (guar) ; } let rhs_tt = p . parse_token_tree () ; let rhs_tt = parse_one_tt (rhs_tt , RulePart :: Body , sess , node_id , features , edition) ; check_emission (check_rhs (sess , & rhs_tt)) ; check_emission (check_meta_variables (& sess . psess , node_id , args . as_ref () , & lhs_tt , & rhs_tt)) ; let lhs_span = lhs_tt . span () ; let lhs = if let mbe :: TokenTree :: Delimited (.. , delimited) = lhs_tt { mbe :: macro_parser :: compute_locs (& delimited . tts) } else { return dummy_syn_ext (guar . unwrap ()) ; } ; if let Some (args) = args { let args_span = args . span () ; let mbe :: TokenTree :: Delimited (.. , delimited) = args else { return dummy_syn_ext (guar . unwrap ()) ; } ; let args = mbe :: macro_parser :: compute_locs (& delimited . tts) ; let body_span = lhs_span ; rules . push (MacroRule :: Attr { args , args_span , body : lhs , body_span , rhs : rhs_tt }) ; } else if is_derive { rules . push (MacroRule :: Derive { body : lhs , body_span : lhs_span , rhs : rhs_tt }) ; } else { rules . push (MacroRule :: Func { lhs , lhs_span , rhs : rhs_tt }) ; } if p . token == token :: Eof { break ; } if let Err (e) = p . expect (exp_sep) { return dummy_syn_ext (e . emit ()) ; } } if rules . is_empty () { let guar = sess . dcx () . span_err (span , "macros must contain at least one rule") ; return dummy_syn_ext (guar) ; } assert ! (! kinds . is_empty ()) ; let transparency = find_attr ! (attrs , AttributeKind :: MacroTransparency (x) => * x) . unwrap_or (Transparency :: fallback (macro_rules)) ; if let Some (guar) = guar { return dummy_syn_ext (guar) ; } let nrules = if is_defined_in_current_crate (node_id) { rules . len () } else { 0 } ; let exp = MacroRulesMacroExpander { name : ident , kinds , span , node_id , transparency , rules } ; (mk_syn_ext (SyntaxExtensionKind :: MacroRules (Arc :: new (exp))) , nrules) }
}

macro_rules! check_no_eof_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_no_eof in module {}", module_path!());
    };
}

mkfn!{
    check_no_eof_introspect!();
    fn check_no_eof (sess : & Session , p : & Parser < '_ > , msg : & 'static str) -> Option < ErrorGuaranteed > { if p . token == token :: Eof { let err_sp = p . token . span . shrink_to_hi () ; let guar = sess . dcx () . struct_span_err (err_sp , "macro definition ended unexpectedly") . with_span_label (err_sp , msg) . emit () ; return Some (guar) ; } None }
}

macro_rules! check_args_parens_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_args_parens in module {}", module_path!());
    };
}

mkfn!{
    check_args_parens_introspect!();
    fn check_args_parens (sess : & Session , rule_kw : Symbol , args : & tokenstream :: TokenTree) { if let tokenstream :: TokenTree :: Delimited (dspan , _ , delim , _) = args && * delim != Delimiter :: Parenthesis { sess . dcx () . emit_err (errors :: MacroArgsBadDelim { span : dspan . entire () , sugg : errors :: MacroArgsBadDelimSugg { open : dspan . open , close : dspan . close } , rule_kw , }) ; } }
}

macro_rules! check_args_empty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_args_empty in module {}", module_path!());
    };
}

mkfn!{
    check_args_empty_introspect!();
    fn check_args_empty (sess : & Session , args : & tokenstream :: TokenTree) -> Result < () , ErrorGuaranteed > { match args { tokenstream :: TokenTree :: Delimited (.. , delimited) if delimited . is_empty () => Ok (()) , _ => { let msg = "`derive` rules do not accept arguments; `derive` must be followed by `()`" ; Err (sess . dcx () . span_err (args . span () , msg)) } } }
}

macro_rules! check_lhs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_lhs in module {}", module_path!());
    };
}

mkfn!{
    check_lhs_introspect!();
    fn check_lhs (sess : & Session , node_id : NodeId , lhs : & mbe :: TokenTree) -> Result < () , ErrorGuaranteed > { let e1 = check_lhs_nt_follows (sess , node_id , lhs) ; let e2 = check_lhs_no_empty_seq (sess , slice :: from_ref (lhs)) ; e1 . and (e2) }
}

macro_rules! check_lhs_nt_follows_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_lhs_nt_follows in module {}", module_path!());
    };
}

mkfn!{
    check_lhs_nt_follows_introspect!();
    fn check_lhs_nt_follows (sess : & Session , node_id : NodeId , lhs : & mbe :: TokenTree ,) -> Result < () , ErrorGuaranteed > { if let mbe :: TokenTree :: Delimited (.. , delimited) = lhs { check_matcher (sess , node_id , & delimited . tts) } else { let msg = "invalid macro matcher; matchers must be contained in balanced delimiters" ; Err (sess . dcx () . span_err (lhs . span () , msg)) } }
}

macro_rules! is_empty_token_tree_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_empty_token_tree in module {}", module_path!());
    };
}

mkfn!{
    is_empty_token_tree_introspect!();
    fn is_empty_token_tree (sess : & Session , seq : & mbe :: SequenceRepetition) -> bool { if seq . separator . is_some () { false } else { let mut is_empty = true ; let mut iter = seq . tts . iter () . peekable () ; while let Some (tt) = iter . next () { match tt { mbe :: TokenTree :: MetaVarDecl { kind : NonterminalKind :: Vis , .. } => { } mbe :: TokenTree :: Token (t @ Token { kind : DocComment (..) , .. }) => { let mut now = t ; while let Some (& mbe :: TokenTree :: Token (next @ Token { kind : DocComment (..) , .. } ,)) = iter . peek () { now = next ; iter . next () ; } let span = t . span . to (now . span) ; sess . dcx () . span_note (span , "doc comments are ignored in matcher position") ; } mbe :: TokenTree :: Sequence (_ , sub_seq) if (sub_seq . kleene . op == mbe :: KleeneOp :: ZeroOrMore || sub_seq . kleene . op == mbe :: KleeneOp :: ZeroOrOne) => { } _ => is_empty = false , } } is_empty } }
}

macro_rules! check_redundant_vis_repetition_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_redundant_vis_repetition in module {}", module_path!());
    };
}

mkfn!{
    check_redundant_vis_repetition_introspect!();
    # [doc = " Checks if a `vis` nonterminal fragment is unnecessarily wrapped in an optional repetition."] # [doc = ""] # [doc = " When a `vis` fragment (which can already be empty) is wrapped in `$(...)?`,"] # [doc = " this suggests removing the redundant repetition syntax since it provides no additional benefit."] fn check_redundant_vis_repetition (err : & mut Diag < '_ > , sess : & Session , seq : & SequenceRepetition , span : & DelimSpan ,) { let is_zero_or_one : bool = seq . kleene . op == KleeneOp :: ZeroOrOne ; let is_vis = seq . tts . first () . map_or (false , | tt | { matches ! (tt , mbe :: TokenTree :: MetaVarDecl { kind : NonterminalKind :: Vis , .. }) }) ; if is_vis && is_zero_or_one { err . note ("a `vis` fragment can already be empty") ; err . multipart_suggestion ("remove the `$(` and `)?`" , vec ! [(sess . source_map () . span_extend_to_prev_char_before (span . open , '$' , true) , "" . to_string () ,) , (span . close . with_hi (seq . kleene . span . hi ()) , "" . to_string ()) ,] , Applicability :: MaybeIncorrect ,) ; } }
}

macro_rules! check_lhs_no_empty_seq_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_lhs_no_empty_seq in module {}", module_path!());
    };
}

mkfn!{
    check_lhs_no_empty_seq_introspect!();
    # [doc = " Checks that the lhs contains no repetition which could match an empty token"] # [doc = " tree, because then the matcher would hang indefinitely."] fn check_lhs_no_empty_seq (sess : & Session , tts : & [mbe :: TokenTree]) -> Result < () , ErrorGuaranteed > { use mbe :: TokenTree ; for tt in tts { match tt { TokenTree :: Token (..) | TokenTree :: MetaVar (..) | TokenTree :: MetaVarDecl { .. } | TokenTree :: MetaVarExpr (..) => () , TokenTree :: Delimited (.. , del) => check_lhs_no_empty_seq (sess , & del . tts) ? , TokenTree :: Sequence (span , seq) => { if is_empty_token_tree (sess , seq) { let sp = span . entire () ; let mut err = sess . dcx () . struct_span_err (sp , "repetition matches empty token tree") ; check_redundant_vis_repetition (& mut err , sess , seq , span) ; return Err (err . emit ()) ; } check_lhs_no_empty_seq (sess , & seq . tts) ? } } } Ok (()) }
}

macro_rules! check_rhs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_rhs in module {}", module_path!());
    };
}

mkfn!{
    check_rhs_introspect!();
    fn check_rhs (sess : & Session , rhs : & mbe :: TokenTree) -> Result < () , ErrorGuaranteed > { match * rhs { mbe :: TokenTree :: Delimited (..) => Ok (()) , _ => Err (sess . dcx () . span_err (rhs . span () , "macro rhs must be delimited")) , } }
}

macro_rules! check_matcher_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_matcher in module {}", module_path!());
    };
}

mkfn!{
    check_matcher_introspect!();
    fn check_matcher (sess : & Session , node_id : NodeId , matcher : & [mbe :: TokenTree] ,) -> Result < () , ErrorGuaranteed > { let first_sets = FirstSets :: new (matcher) ; let empty_suffix = TokenSet :: empty () ; check_matcher_core (sess , node_id , & first_sets , matcher , & empty_suffix) ? ; Ok (()) }
}

macro_rules! has_compile_error_macro_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_compile_error_macro in module {}", module_path!());
    };
}

mkfn!{
    has_compile_error_macro_introspect!();
    fn has_compile_error_macro (rhs : & mbe :: TokenTree) -> bool { match rhs { mbe :: TokenTree :: Delimited (.. , d) => { let has_compile_error = d . tts . array_windows :: < 3 > () . any (| [ident , bang , args] | { if let mbe :: TokenTree :: Token (ident) = ident && let TokenKind :: Ident (ident , _) = ident . kind && ident == sym :: compile_error && let mbe :: TokenTree :: Token (bang) = bang && let TokenKind :: Bang = bang . kind && let mbe :: TokenTree :: Delimited (.. , del) = args && ! del . delim . skip () { true } else { false } }) ; if has_compile_error { true } else { d . tts . iter () . any (has_compile_error_macro) } } _ => false , } }
}
mkitem!{mkstruct!{struct FirstSets < 'tt > { first : FxHashMap < Span , Option < TokenSet < 'tt > > > , }}}
mkitem!{mkimpl!{impl < 'tt > FirstSets < 'tt > { fn new (tts : & 'tt [mbe :: TokenTree]) -> FirstSets < 'tt > { use mbe :: TokenTree ; let mut sets = FirstSets { first : FxHashMap :: default () } ; build_recur (& mut sets , tts) ; return sets ; fn build_recur < 'tt > (sets : & mut FirstSets < 'tt > , tts : & 'tt [TokenTree]) -> TokenSet < 'tt > { let mut first = TokenSet :: empty () ; for tt in tts . iter () . rev () { match tt { TokenTree :: Token (..) | TokenTree :: MetaVar (..) | TokenTree :: MetaVarDecl { .. } | TokenTree :: MetaVarExpr (..) => { first . replace_with (TtHandle :: TtRef (tt)) ; } TokenTree :: Delimited (span , _ , delimited) => { build_recur (sets , & delimited . tts) ; first . replace_with (TtHandle :: from_token_kind (delimited . delim . as_open_token_kind () , span . open ,)) ; } TokenTree :: Sequence (sp , seq_rep) => { let subfirst = build_recur (sets , & seq_rep . tts) ; match sets . first . entry (sp . entire ()) { Entry :: Vacant (vac) => { vac . insert (Some (subfirst . clone ())) ; } Entry :: Occupied (mut occ) => { occ . insert (None) ; } } if let (Some (sep) , true) = (& seq_rep . separator , subfirst . maybe_empty) { first . add_one_maybe (TtHandle :: from_token (* sep)) ; } if subfirst . maybe_empty || seq_rep . kleene . op == mbe :: KleeneOp :: ZeroOrMore || seq_rep . kleene . op == mbe :: KleeneOp :: ZeroOrOne { first . add_all (& TokenSet { maybe_empty : true , .. subfirst }) ; } else { first = subfirst ; } } } } first } } fn first (& self , tts : & 'tt [mbe :: TokenTree]) -> TokenSet < 'tt > { use mbe :: TokenTree ; let mut first = TokenSet :: empty () ; for tt in tts . iter () { assert ! (first . maybe_empty) ; match tt { TokenTree :: Token (..) | TokenTree :: MetaVar (..) | TokenTree :: MetaVarDecl { .. } | TokenTree :: MetaVarExpr (..) => { first . add_one (TtHandle :: TtRef (tt)) ; return first ; } TokenTree :: Delimited (span , _ , delimited) => { first . add_one (TtHandle :: from_token_kind (delimited . delim . as_open_token_kind () , span . open ,)) ; return first ; } TokenTree :: Sequence (sp , seq_rep) => { let subfirst_owned ; let subfirst = match self . first . get (& sp . entire ()) { Some (Some (subfirst)) => subfirst , Some (& None) => { subfirst_owned = self . first (& seq_rep . tts) ; & subfirst_owned } None => { panic ! ("We missed a sequence during FirstSets construction") ; } } ; if let (Some (sep) , true) = (& seq_rep . separator , subfirst . maybe_empty) { first . add_one_maybe (TtHandle :: from_token (* sep)) ; } assert ! (first . maybe_empty) ; first . add_all (subfirst) ; if subfirst . maybe_empty || seq_rep . kleene . op == mbe :: KleeneOp :: ZeroOrMore || seq_rep . kleene . op == mbe :: KleeneOp :: ZeroOrOne { first . maybe_empty = true ; continue ; } else { return first ; } } } } assert ! (first . maybe_empty) ; first } }}}
mkitem!{mkenum!{# [derive (Debug)] enum TtHandle < 'tt > { # [doc = " This is used in most cases."] TtRef (& 'tt mbe :: TokenTree) , # [doc = " This is only used for implicit token trees. The `mbe::TokenTree` *must*"] # [doc = " be `mbe::TokenTree::Token`. No other variants are allowed. We store an"] # [doc = " `mbe::TokenTree` rather than a `Token` so that `get()` can return a"] # [doc = " `&mbe::TokenTree`."] Token (mbe :: TokenTree) , }}}
mkitem!{mkimpl!{impl < 'tt > TtHandle < 'tt > { fn from_token (tok : Token) -> Self { TtHandle :: Token (mbe :: TokenTree :: Token (tok)) } fn from_token_kind (kind : TokenKind , span : Span) -> Self { TtHandle :: from_token (Token :: new (kind , span)) } fn get (& 'tt self) -> & 'tt mbe :: TokenTree { match self { TtHandle :: TtRef (tt) => tt , TtHandle :: Token (token_tt) => token_tt , } } }}}
mkitem!{mkimpl!{impl < 'tt > PartialEq for TtHandle < 'tt > { fn eq (& self , other : & TtHandle < 'tt >) -> bool { self . get () == other . get () } }}}
mkitem!{mkimpl!{impl < 'tt > Clone for TtHandle < 'tt > { fn clone (& self) -> Self { match self { TtHandle :: TtRef (tt) => TtHandle :: TtRef (tt) , TtHandle :: Token (mbe :: TokenTree :: Token (tok)) => { TtHandle :: Token (mbe :: TokenTree :: Token (* tok)) } _ => unreachable ! () , } } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug)] struct TokenSet < 'tt > { tokens : Vec < TtHandle < 'tt > > , maybe_empty : bool , }}}
mkitem!{mkimpl!{impl < 'tt > TokenSet < 'tt > { fn empty () -> Self { TokenSet { tokens : Vec :: new () , maybe_empty : true } } fn singleton (tt : TtHandle < 'tt >) -> Self { TokenSet { tokens : vec ! [tt] , maybe_empty : false } } fn replace_with (& mut self , tt : TtHandle < 'tt >) { self . tokens . clear () ; self . tokens . push (tt) ; self . maybe_empty = false ; } fn replace_with_irrelevant (& mut self) { self . tokens . clear () ; self . maybe_empty = false ; } fn add_one (& mut self , tt : TtHandle < 'tt >) { if ! self . tokens . contains (& tt) { self . tokens . push (tt) ; } self . maybe_empty = false ; } fn add_one_maybe (& mut self , tt : TtHandle < 'tt >) { if ! self . tokens . contains (& tt) { self . tokens . push (tt) ; } } fn add_all (& mut self , other : & Self) { for tt in & other . tokens { if ! self . tokens . contains (tt) { self . tokens . push (tt . clone ()) ; } } if ! other . maybe_empty { self . maybe_empty = false ; } } }}}

macro_rules! check_matcher_core_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_matcher_core in module {}", module_path!());
    };
}

mkfn!{
    check_matcher_core_introspect!();
    fn check_matcher_core < 'tt > (sess : & Session , node_id : NodeId , first_sets : & FirstSets < 'tt > , matcher : & 'tt [mbe :: TokenTree] , follow : & TokenSet < 'tt > ,) -> Result < TokenSet < 'tt > , ErrorGuaranteed > { use mbe :: TokenTree ; let mut last = TokenSet :: empty () ; let mut errored = Ok (()) ; 'each_token : for i in 0 .. matcher . len () { let token = & matcher [i] ; let suffix = & matcher [i + 1 ..] ; let build_suffix_first = | | { let mut s = first_sets . first (suffix) ; if s . maybe_empty { s . add_all (follow) ; } s } ; let suffix_first ; match token { TokenTree :: Token (..) | TokenTree :: MetaVar (..) | TokenTree :: MetaVarDecl { .. } | TokenTree :: MetaVarExpr (..) => { if token_can_be_followed_by_any (token) { last . replace_with_irrelevant () ; continue 'each_token ; } else { last . replace_with (TtHandle :: TtRef (token)) ; suffix_first = build_suffix_first () ; } } TokenTree :: Delimited (span , _ , d) => { let my_suffix = TokenSet :: singleton (TtHandle :: from_token_kind (d . delim . as_close_token_kind () , span . close ,)) ; check_matcher_core (sess , node_id , first_sets , & d . tts , & my_suffix) ? ; last . replace_with_irrelevant () ; continue 'each_token ; } TokenTree :: Sequence (_ , seq_rep) => { suffix_first = build_suffix_first () ; let mut new ; let my_suffix = if let Some (sep) = & seq_rep . separator { new = suffix_first . clone () ; new . add_one_maybe (TtHandle :: from_token (* sep)) ; & new } else { & suffix_first } ; let next = check_matcher_core (sess , node_id , first_sets , & seq_rep . tts , my_suffix) ? ; if next . maybe_empty { last . add_all (& next) ; } else { last = next ; } continue 'each_token ; } } for tt in & last . tokens { if let & TokenTree :: MetaVarDecl { span , name , kind } = tt . get () { for next_token in & suffix_first . tokens { let next_token = next_token . get () ; if is_defined_in_current_crate (node_id) && matches ! (kind , NonterminalKind :: Pat (PatParam { inferred : true })) && matches ! (next_token , TokenTree :: Token (token) if * token == token :: Or) { let suggestion = quoted_tt_to_string (& TokenTree :: MetaVarDecl { span , name , kind : NonterminalKind :: Pat (PatParam { inferred : false }) , }) ; sess . psess . buffer_lint (RUST_2021_INCOMPATIBLE_OR_PATTERNS , span , ast :: CRATE_NODE_ID , BuiltinLintDiag :: OrPatternsBackCompat (span , suggestion) ,) ; } match is_in_follow (next_token , kind) { IsInFollow :: Yes => { } IsInFollow :: No (possible) => { let may_be = if last . tokens . len () == 1 && suffix_first . tokens . len () == 1 { "is" } else { "may be" } ; let sp = next_token . span () ; let mut err = sess . dcx () . struct_span_err (sp , format ! ("`${name}:{frag}` {may_be} followed by `{next}`, which \
                                     is not allowed for `{frag}` fragments" , name = name , frag = kind , next = quoted_tt_to_string (next_token) , may_be = may_be) ,) ; err . span_label (sp , format ! ("not allowed after `{kind}` fragments")) ; if kind == NonterminalKind :: Pat (PatWithOr) && sess . psess . edition . at_least_rust_2021 () && next_token . is_token (& token :: Or) { let suggestion = quoted_tt_to_string (& TokenTree :: MetaVarDecl { span , name , kind : NonterminalKind :: Pat (PatParam { inferred : false }) , }) ; err . span_suggestion (span , "try a `pat_param` fragment specifier instead" , suggestion , Applicability :: MaybeIncorrect ,) ; } let msg = "allowed there are: " ; match possible { & [] => { } & [t] => { err . note (format ! ("only {t} is allowed after `{kind}` fragments" ,)) ; } ts => { err . note (format ! ("{}{} or {}" , msg , ts [.. ts . len () - 1] . to_vec () . join (", ") , ts [ts . len () - 1] ,)) ; } } errored = Err (err . emit ()) ; } } } } } } errored ? ; Ok (last) }
}

macro_rules! token_can_be_followed_by_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function token_can_be_followed_by_any in module {}", module_path!());
    };
}

mkfn!{
    token_can_be_followed_by_any_introspect!();
    fn token_can_be_followed_by_any (tok : & mbe :: TokenTree) -> bool { if let mbe :: TokenTree :: MetaVarDecl { kind , .. } = * tok { frag_can_be_followed_by_any (kind) } else { true } }
}

macro_rules! frag_can_be_followed_by_any_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function frag_can_be_followed_by_any in module {}", module_path!());
    };
}

mkfn!{
    frag_can_be_followed_by_any_introspect!();
    # [doc = " Returns `true` if a fragment of type `frag` can be followed by any sort of"] # [doc = " token. We use this (among other things) as a useful approximation"] # [doc = " for when `frag` can be followed by a repetition like `$(...)*` or"] # [doc = " `$(...)+`. In general, these can be a bit tricky to reason about,"] # [doc = " so we adopt a conservative position that says that any fragment"] # [doc = " specifier which consumes at most one token tree can be followed by"] # [doc = " a fragment specifier (indeed, these fragments can be followed by"] # [doc = " ANYTHING without fear of future compatibility hazards)."] fn frag_can_be_followed_by_any (kind : NonterminalKind) -> bool { matches ! (kind , NonterminalKind :: Item | NonterminalKind :: Block | NonterminalKind :: Ident | NonterminalKind :: Literal | NonterminalKind :: Meta | NonterminalKind :: Lifetime | NonterminalKind :: TT) }
}
mkitem!{mkenum!{enum IsInFollow { Yes , No (& 'static [& 'static str]) , }}}

macro_rules! is_in_follow_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_in_follow in module {}", module_path!());
    };
}

mkfn!{
    is_in_follow_introspect!();
    # [doc = " Returns `true` if `frag` can legally be followed by the token `tok`. For"] # [doc = " fragments that can consume an unbounded number of tokens, `tok`"] # [doc = " must be within a well-defined follow set. This is intended to"] # [doc = " guarantee future compatibility: for example, without this rule, if"] # [doc = " we expanded `expr` to include a new binary operator, we might"] # [doc = " break macros that were relying on that binary operator as a"] # [doc = " separator."] fn is_in_follow (tok : & mbe :: TokenTree , kind : NonterminalKind) -> IsInFollow { use mbe :: TokenTree ; if let TokenTree :: Token (Token { kind , .. }) = tok && kind . close_delim () . is_some () { IsInFollow :: Yes } else { match kind { NonterminalKind :: Item => { IsInFollow :: Yes } NonterminalKind :: Block => { IsInFollow :: Yes } NonterminalKind :: Stmt | NonterminalKind :: Expr (_) => { const TOKENS : & [& str] = & ["`=>`" , "`,`" , "`;`"] ; match tok { TokenTree :: Token (token) => match token . kind { FatArrow | Comma | Semi => IsInFollow :: Yes , _ => IsInFollow :: No (TOKENS) , } , _ => IsInFollow :: No (TOKENS) , } } NonterminalKind :: Pat (PatParam { .. }) => { const TOKENS : & [& str] = & ["`=>`" , "`,`" , "`=`" , "`|`" , "`if`" , "`in`"] ; match tok { TokenTree :: Token (token) => match token . kind { FatArrow | Comma | Eq | Or => IsInFollow :: Yes , Ident (name , IdentIsRaw :: No) if name == kw :: If || name == kw :: In => { IsInFollow :: Yes } _ => IsInFollow :: No (TOKENS) , } , _ => IsInFollow :: No (TOKENS) , } } NonterminalKind :: Pat (PatWithOr) => { const TOKENS : & [& str] = & ["`=>`" , "`,`" , "`=`" , "`if`" , "`in`"] ; match tok { TokenTree :: Token (token) => match token . kind { FatArrow | Comma | Eq => IsInFollow :: Yes , Ident (name , IdentIsRaw :: No) if name == kw :: If || name == kw :: In => { IsInFollow :: Yes } _ => IsInFollow :: No (TOKENS) , } , _ => IsInFollow :: No (TOKENS) , } } NonterminalKind :: Path | NonterminalKind :: Ty => { const TOKENS : & [& str] = & ["`{`" , "`[`" , "`=>`" , "`,`" , "`>`" , "`=`" , "`:`" , "`;`" , "`|`" , "`as`" , "`where`" ,] ; match tok { TokenTree :: Token (token) => match token . kind { OpenBrace | OpenBracket | Comma | FatArrow | Colon | Eq | Gt | Shr | Semi | Or => IsInFollow :: Yes , Ident (name , IdentIsRaw :: No) if name == kw :: As || name == kw :: Where => { IsInFollow :: Yes } _ => IsInFollow :: No (TOKENS) , } , TokenTree :: MetaVarDecl { kind : NonterminalKind :: Block , .. } => IsInFollow :: Yes , _ => IsInFollow :: No (TOKENS) , } } NonterminalKind :: Ident | NonterminalKind :: Lifetime => { IsInFollow :: Yes } NonterminalKind :: Literal => { IsInFollow :: Yes } NonterminalKind :: Meta | NonterminalKind :: TT => { IsInFollow :: Yes } NonterminalKind :: Vis => { const TOKENS : & [& str] = & ["`,`" , "an ident" , "a type"] ; match tok { TokenTree :: Token (token) => match token . kind { Comma => IsInFollow :: Yes , Ident (_ , IdentIsRaw :: Yes) => IsInFollow :: Yes , Ident (name , _) if name != kw :: Priv => IsInFollow :: Yes , _ => { if token . can_begin_type () { IsInFollow :: Yes } else { IsInFollow :: No (TOKENS) } } } , TokenTree :: MetaVarDecl { kind : NonterminalKind :: Ident | NonterminalKind :: Ty | NonterminalKind :: Path , .. } => IsInFollow :: Yes , _ => IsInFollow :: No (TOKENS) , } } } } }
}

macro_rules! quoted_tt_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function quoted_tt_to_string in module {}", module_path!());
    };
}

mkfn!{
    quoted_tt_to_string_introspect!();
    fn quoted_tt_to_string (tt : & mbe :: TokenTree) -> String { match tt { mbe :: TokenTree :: Token (token) => pprust :: token_to_string (token) . into () , mbe :: TokenTree :: MetaVar (_ , name) => format ! ("${name}") , mbe :: TokenTree :: MetaVarDecl { name , kind , .. } => format ! ("${name}:{kind}") , _ => panic ! ("{}" , "unexpected mbe::TokenTree::{Sequence or Delimited} \
             in follow set checker") , } }
}

macro_rules! is_defined_in_current_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_defined_in_current_crate in module {}", module_path!());
    };
}

mkfn!{
    is_defined_in_current_crate_introspect!();
    fn is_defined_in_current_crate (node_id : NodeId) -> bool { node_id != DUMMY_NODE_ID }
}

macro_rules! parser_from_cx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parser_from_cx in module {}", module_path!());
    };
}

mkfn!{
    parser_from_cx_introspect!();
    pub (super) fn parser_from_cx (psess : & ParseSess , mut tts : TokenStream , recovery : Recovery ,) -> Parser < '_ > { tts . desugar_doc_comments () ; Parser :: new (psess , tts , rustc_parse :: MACRO_ARGUMENTS) . recovery (recovery) }
}