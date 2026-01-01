/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (rustc :: diagnostic_outside_of_impl)] # [allow (rustc :: untranslatable_diagnostic)] # [feature (assert_matches)] # [feature (box_patterns)] # [feature (debug_closure_helpers)] # [feature (default_field_values)] # [feature (if_let_guard)] # [feature (iter_intersperse)] # [recursion_limit = "256"] use std :: path :: { Path , PathBuf } ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0002
/* FP:lib.rs-0004 */ use std :: str :: Utf8Error ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0003
/* FP:lib.rs-0006 */ use std :: sync :: Arc ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0004
/* FP:lib.rs-0008 */ use rustc_ast as ast ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0005
/* FP:lib.rs-0010 */ use crate :: rustc_complete :: tokenstream :: { DelimSpan , TokenStream } ;
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0006
/* FP:lib.rs-0012 */ use crate :: rustc_complete :: { AttrItem , Attribute , MetaItemInner , token } ;
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0007
/* FP:lib.rs-0014 */ use rustc_ast_pretty :: pprust ;
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0008
/* FP:lib.rs-0016 */ use crate :: rustc_complete :: { Diag , EmissionGuarantee , FatalError , PResult , pluralize } ;
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0009
/* FP:lib.rs-0018 */ use crate :: rustc_complete :: parse :: ParseSess ;
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0010
/* FP:lib.rs-0020 */ use crate :: rustc_complete :: source_map :: SourceMap ;
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0011
/* FP:lib.rs-0022 */ use crate :: rustc_complete :: { FileName , SourceFile , Span } ;
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0012
/* FP:lib.rs-0024 */ pub use unicode_normalization :: UNICODE_VERSION as UNICODE_NORMALIZATION_VERSION ;
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_CONST_0013
/* FP:lib.rs-0026 */ pub const MACRO_ARGUMENTS : Option < & str > = Some ("macro arguments") ;
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_MOD_0014
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0015
/* FP:lib.rs-0030 */ use parser :: Parser ;
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0016
/* FP:lib.rs-0032 */ use crate :: rustc_complete :: token :: Delimiter ;
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_USE_0017
/* FP:lib.rs-0034 */ use crate :: lexer :: StripTokens ;
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_MOD_0018
/* FP:lib.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_MOD_0019
/* FP:lib.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_MACRO_0020
/* FP:lib.rs-0040 */ rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
/* FP:lib.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0021
/* FP:lib.rs-0042 */ pub fn unwrap_or_emit_fatal < T > (expr : Result < T , Vec < Diag < '_ > > >) -> T { match expr { Ok (expr) => expr , Err (errs) => { for err in errs { err . emit () ; } FatalError . raise () } } }
/* FP:lib.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0022
/* FP:lib.rs-0044 */ # [doc = " Creates a new parser from a source string."] # [doc = ""] # [doc = " On failure, the errors must be consumed via `unwrap_or_emit_fatal`, `emit`, `cancel`,"] # [doc = " etc., otherwise a panic will occur when they are dropped."] pub fn new_parser_from_source_str (psess : & ParseSess , name : FileName , source : String , strip_tokens : StripTokens ,) -> Result < Parser < '_ > , Vec < Diag < '_ > > > { let source_file = psess . source_map () . new_source_file (name , source) ; new_parser_from_source_file (psess , source_file , strip_tokens) }
/* FP:lib.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0023
/* FP:lib.rs-0046 */ # [doc = " Creates a new parser from a filename. On failure, the errors must be consumed via"] # [doc = " `unwrap_or_emit_fatal`, `emit`, `cancel`, etc., otherwise a panic will occur when they are"] # [doc = " dropped."] # [doc = ""] # [doc = " If a span is given, that is used on an error as the source of the problem."] pub fn new_parser_from_file < 'a > (psess : & 'a ParseSess , path : & Path , strip_tokens : StripTokens , sp : Option < Span > ,) -> Result < Parser < 'a > , Vec < Diag < 'a > > > { let sm = psess . source_map () ; let source_file = sm . load_file (path) . unwrap_or_else (| e | { let msg = format ! ("couldn't read `{}`: {}" , path . display () , e) ; let mut err = psess . dcx () . struct_fatal (msg) ; if let Ok (contents) = std :: fs :: read (path) && let Err (utf8err) = String :: from_utf8 (contents . clone ()) { utf8_error (sm , & path . display () . to_string () , sp , & mut err , utf8err . utf8_error () , & contents ,) ; } if let Some (sp) = sp { err . span (sp) ; } err . emit () ; }) ; new_parser_from_source_file (psess , source_file , strip_tokens) }
/* FP:lib.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0024
/* FP:lib.rs-0048 */ pub fn utf8_error < E : EmissionGuarantee > (sm : & SourceMap , path : & str , sp : Option < Span > , err : & mut Diag < '_ , E > , utf8err : Utf8Error , contents : & [u8] ,) { let start = utf8err . valid_up_to () ; let note = format ! ("invalid utf-8 at byte `{start}`") ; let msg = if let Some (len) = utf8err . error_len () { format ! ("byte{s} `{bytes}` {are} not valid utf-8" , bytes = if len == 1 { format ! ("{:?}" , contents [start]) } else { format ! ("{:?}" , & contents [start .. start + len]) } , s = pluralize ! (len) , are = if len == 1 { "is" } else { "are" } ,) } else { note . clone () } ; let contents = String :: from_utf8_lossy (contents) . to_string () ; let source = sm . new_source_file (PathBuf :: from (path) . into () , contents) ; let span = Span :: with_root_ctxt (source . normalized_byte_pos (start as u32) , source . normalized_byte_pos (start as u32) ,) ; if span . is_dummy () { err . note (note) ; } else { if sp . is_some () { err . span_note (span , msg) ; } else { err . span (span) ; err . span_label (span , msg) ; } } }
/* FP:lib.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0025
/* FP:lib.rs-0050 */ # [doc = " Given a session and a `source_file`, return a parser. Returns any buffered errors from lexing"] # [doc = " the initial token stream."] fn new_parser_from_source_file (psess : & ParseSess , source_file : Arc < SourceFile > , strip_tokens : StripTokens ,) -> Result < Parser < '_ > , Vec < Diag < '_ > > > { let end_pos = source_file . end_position () ; let stream = source_file_to_stream (psess , source_file , None , strip_tokens) ? ; let mut parser = Parser :: new (psess , stream , None) ; if parser . token == token :: Eof { parser . token . span = Span :: new (end_pos , end_pos , parser . token . span . ctxt () , None) ; } Ok (parser) }
/* FP:lib.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0026
/* FP:lib.rs-0052 */ # [doc = " Given a source string, produces a sequence of token trees."] # [doc = ""] # [doc = " NOTE: This only strips shebangs, not frontmatter!"] pub fn source_str_to_stream (psess : & ParseSess , name : FileName , source : String , override_span : Option < Span > ,) -> Result < TokenStream , Vec < Diag < '_ > > > { let source_file = psess . source_map () . new_source_file (name , source) ; source_file_to_stream (psess , source_file , override_span , StripTokens :: Shebang) }
/* FP:lib.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0027
/* FP:lib.rs-0054 */ # [doc = " Given a source file, produces a sequence of token trees."] # [doc = ""] # [doc = " Returns any buffered errors from parsing the token stream."] fn source_file_to_stream < 'psess > (psess : & 'psess ParseSess , source_file : Arc < SourceFile > , override_span : Option < Span > , strip_tokens : StripTokens ,) -> Result < TokenStream , Vec < Diag < 'psess > > > { let src = source_file . src . as_ref () . unwrap_or_else (| | { psess . dcx () . bug (format ! ("cannot lex `source_file` without source: {}" , psess . source_map () . filename_for_diagnostics (& source_file . name))) ; }) ; lexer :: lex_token_trees (psess , src . as_str () , source_file . start_pos , override_span , strip_tokens) }
/* FP:lib.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0028
/* FP:lib.rs-0056 */ # [doc = " Runs the given subparser `f` on the tokens of the given `attr`'s item."] pub fn parse_in < 'a , T > (psess : & 'a ParseSess , tts : TokenStream , name : & 'static str , mut f : impl FnMut (& mut Parser < 'a >) -> PResult < 'a , T > ,) -> PResult < 'a , T > { let mut parser = Parser :: new (psess , tts , Some (name)) ; let result = f (& mut parser) ? ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok (result) }
/* FP:lib.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0029
/* FP:lib.rs-0058 */ pub fn fake_token_stream_for_item (psess : & ParseSess , item : & ast :: Item) -> TokenStream { let source = pprust :: item_to_string (item) ; let filename = FileName :: macro_expansion_source_code (& source) ; unwrap_or_emit_fatal (source_str_to_stream (psess , filename , source , Some (item . span))) }
/* FP:lib.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0030
/* FP:lib.rs-0060 */ pub fn fake_token_stream_for_crate (psess : & ParseSess , krate : & ast :: Crate) -> TokenStream { let source = pprust :: crate_to_string_for_macros (krate) ; let filename = FileName :: macro_expansion_source_code (& source) ; unwrap_or_emit_fatal (source_str_to_stream (psess , filename , source , Some (krate . spans . inner_span) ,)) }
/* FP:lib.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0031
/* FP:lib.rs-0062 */ pub fn parse_cfg_attr (cfg_attr : & Attribute , psess : & ParseSess ,) -> Option < (MetaItemInner , Vec < (AttrItem , Span) >) > { const CFG_ATTR_GRAMMAR_HELP : & str = "#[cfg_attr(condition, attribute, other_attribute, ...)]" ; const CFG_ATTR_NOTE_REF : & str = "for more information, visit \
/* FP:lib.rs-0063 */         <https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute>" ; match cfg_attr . get_normal_item () . args { ast :: AttrArgs :: Delimited (ast :: DelimArgs { dspan , delim , ref tokens }) if ! tokens . is_empty () => { check_cfg_attr_bad_delim (psess , dspan , delim) ; match parse_in (psess , tokens . clone () , "`cfg_attr` input" , | p | p . parse_cfg_attr ()) { Ok (r) => return Some (r) , Err (e) => { e . with_help (format ! ("the valid syntax is `{CFG_ATTR_GRAMMAR_HELP}`")) . with_note (CFG_ATTR_NOTE_REF) . emit () ; } } } _ => { psess . dcx () . emit_err (errors :: MalformedCfgAttr { span : cfg_attr . span , sugg : CFG_ATTR_GRAMMAR_HELP , }) ; } } None }
/* FP:lib.rs-0064 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_parse_src_lib_FN_0032
/* FP:lib.rs-0065 */ fn check_cfg_attr_bad_delim (psess : & ParseSess , span : DelimSpan , delim : Delimiter) { if let Delimiter :: Parenthesis = delim { return ; } psess . dcx () . emit_err (errors :: CfgAttrBadDelim { span : span . entire () , sugg : errors :: MetaBadDelimSugg { open : span . open , close : span . close } , }) ; }