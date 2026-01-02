mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: str :: Utf8Error ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: tokenstream :: { DelimSpan , TokenStream } ;}
mkuse!{use rustc_ast :: { AttrItem , Attribute , MetaItemInner , token } ;}
mkuse!{use rustc_ast_pretty :: pprust ;}
mkuse!{use rustc_errors :: { Diag , EmissionGuarantee , FatalError , PResult , pluralize } ;}
mkuse!{use rustc_session :: parse :: ParseSess ;}
mkuse!{use rustc_span :: source_map :: SourceMap ;}
mkuse!{use rustc_span :: { FileName , SourceFile , Span } ;}
mkuse!{pub use unicode_normalization :: UNICODE_VERSION as UNICODE_NORMALIZATION_VERSION ;}
mkitem!{pub const MACRO_ARGUMENTS : Option < & str > = Some ("macro arguments") ;}
mkmod!{parser, { 
                getname!(parser);
                getsrc!(parser);
                getpath!(parser);
                get_deps!(parser);
                get_crates!(parser);
                mkinclude!(parser);
                 
            }}
mkuse!{use parser :: Parser ;}
mkuse!{use rustc_ast :: token :: Delimiter ;}
mkuse!{use crate :: lexer :: StripTokens ;}
mkmod!{lexer, { 
                getname!(lexer);
                getsrc!(lexer);
                getpath!(lexer);
                get_deps!(lexer);
                get_crates!(lexer);
                mkinclude!(lexer);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! unwrap_or_emit_fatal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unwrap_or_emit_fatal in module {}", module_path!());
    };
}

mkfn!{
    unwrap_or_emit_fatal_introspect!();
    pub fn unwrap_or_emit_fatal < T > (expr : Result < T , Vec < Diag < '_ > > >) -> T { match expr { Ok (expr) => expr , Err (errs) => { for err in errs { err . emit () ; } FatalError . raise () } } }
}

macro_rules! new_parser_from_source_str_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_parser_from_source_str in module {}", module_path!());
    };
}

mkfn!{
    new_parser_from_source_str_introspect!();
    # [doc = " Creates a new parser from a source string."] # [doc = ""] # [doc = " On failure, the errors must be consumed via `unwrap_or_emit_fatal`, `emit`, `cancel`,"] # [doc = " etc., otherwise a panic will occur when they are dropped."] pub fn new_parser_from_source_str (psess : & ParseSess , name : FileName , source : String , strip_tokens : StripTokens ,) -> Result < Parser < '_ > , Vec < Diag < '_ > > > { let source_file = psess . source_map () . new_source_file (name , source) ; new_parser_from_source_file (psess , source_file , strip_tokens) }
}

macro_rules! new_parser_from_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_parser_from_file in module {}", module_path!());
    };
}

mkfn!{
    new_parser_from_file_introspect!();
    # [doc = " Creates a new parser from a filename. On failure, the errors must be consumed via"] # [doc = " `unwrap_or_emit_fatal`, `emit`, `cancel`, etc., otherwise a panic will occur when they are"] # [doc = " dropped."] # [doc = ""] # [doc = " If a span is given, that is used on an error as the source of the problem."] pub fn new_parser_from_file < 'a > (psess : & 'a ParseSess , path : & Path , strip_tokens : StripTokens , sp : Option < Span > ,) -> Result < Parser < 'a > , Vec < Diag < 'a > > > { let sm = psess . source_map () ; let source_file = sm . load_file (path) . unwrap_or_else (| e | { let msg = format ! ("couldn't read `{}`: {}" , path . display () , e) ; let mut err = psess . dcx () . struct_fatal (msg) ; if let Ok (contents) = std :: fs :: read (path) && let Err (utf8err) = String :: from_utf8 (contents . clone ()) { utf8_error (sm , & path . display () . to_string () , sp , & mut err , utf8err . utf8_error () , & contents ,) ; } if let Some (sp) = sp { err . span (sp) ; } err . emit () ; }) ; new_parser_from_source_file (psess , source_file , strip_tokens) }
}

macro_rules! utf8_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function utf8_error in module {}", module_path!());
    };
}

mkfn!{
    utf8_error_introspect!();
    pub fn utf8_error < E : EmissionGuarantee > (sm : & SourceMap , path : & str , sp : Option < Span > , err : & mut Diag < '_ , E > , utf8err : Utf8Error , contents : & [u8] ,) { let start = utf8err . valid_up_to () ; let note = format ! ("invalid utf-8 at byte `{start}`") ; let msg = if let Some (len) = utf8err . error_len () { format ! ("byte{s} `{bytes}` {are} not valid utf-8" , bytes = if len == 1 { format ! ("{:?}" , contents [start]) } else { format ! ("{:?}" , & contents [start .. start + len]) } , s = pluralize ! (len) , are = if len == 1 { "is" } else { "are" } ,) } else { note . clone () } ; let contents = String :: from_utf8_lossy (contents) . to_string () ; let source = sm . new_source_file (PathBuf :: from (path) . into () , contents) ; let span = Span :: with_root_ctxt (source . normalized_byte_pos (start as u32) , source . normalized_byte_pos (start as u32) ,) ; if span . is_dummy () { err . note (note) ; } else { if sp . is_some () { err . span_note (span , msg) ; } else { err . span (span) ; err . span_label (span , msg) ; } } }
}

macro_rules! new_parser_from_source_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_parser_from_source_file in module {}", module_path!());
    };
}

mkfn!{
    new_parser_from_source_file_introspect!();
    # [doc = " Given a session and a `source_file`, return a parser. Returns any buffered errors from lexing"] # [doc = " the initial token stream."] fn new_parser_from_source_file (psess : & ParseSess , source_file : Arc < SourceFile > , strip_tokens : StripTokens ,) -> Result < Parser < '_ > , Vec < Diag < '_ > > > { let end_pos = source_file . end_position () ; let stream = source_file_to_stream (psess , source_file , None , strip_tokens) ? ; let mut parser = Parser :: new (psess , stream , None) ; if parser . token == token :: Eof { parser . token . span = Span :: new (end_pos , end_pos , parser . token . span . ctxt () , None) ; } Ok (parser) }
}

macro_rules! source_str_to_stream_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function source_str_to_stream in module {}", module_path!());
    };
}

mkfn!{
    source_str_to_stream_introspect!();
    # [doc = " Given a source string, produces a sequence of token trees."] # [doc = ""] # [doc = " NOTE: This only strips shebangs, not frontmatter!"] pub fn source_str_to_stream (psess : & ParseSess , name : FileName , source : String , override_span : Option < Span > ,) -> Result < TokenStream , Vec < Diag < '_ > > > { let source_file = psess . source_map () . new_source_file (name , source) ; source_file_to_stream (psess , source_file , override_span , StripTokens :: Shebang) }
}

macro_rules! source_file_to_stream_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function source_file_to_stream in module {}", module_path!());
    };
}

mkfn!{
    source_file_to_stream_introspect!();
    # [doc = " Given a source file, produces a sequence of token trees."] # [doc = ""] # [doc = " Returns any buffered errors from parsing the token stream."] fn source_file_to_stream < 'psess > (psess : & 'psess ParseSess , source_file : Arc < SourceFile > , override_span : Option < Span > , strip_tokens : StripTokens ,) -> Result < TokenStream , Vec < Diag < 'psess > > > { let src = source_file . src . as_ref () . unwrap_or_else (| | { psess . dcx () . bug (format ! ("cannot lex `source_file` without source: {}" , psess . source_map () . filename_for_diagnostics (& source_file . name))) ; }) ; lexer :: lex_token_trees (psess , src . as_str () , source_file . start_pos , override_span , strip_tokens) }
}

macro_rules! parse_in_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_in in module {}", module_path!());
    };
}

mkfn!{
    parse_in_introspect!();
    # [doc = " Runs the given subparser `f` on the tokens of the given `attr`'s item."] pub fn parse_in < 'a , T > (psess : & 'a ParseSess , tts : TokenStream , name : & 'static str , mut f : impl FnMut (& mut Parser < 'a >) -> PResult < 'a , T > ,) -> PResult < 'a , T > { let mut parser = Parser :: new (psess , tts , Some (name)) ; let result = f (& mut parser) ? ; if parser . token != token :: Eof { parser . unexpected () ? ; } Ok (result) }
}

macro_rules! fake_token_stream_for_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fake_token_stream_for_item in module {}", module_path!());
    };
}

mkfn!{
    fake_token_stream_for_item_introspect!();
    pub fn fake_token_stream_for_item (psess : & ParseSess , item : & ast :: Item) -> TokenStream { let source = pprust :: item_to_string (item) ; let filename = FileName :: macro_expansion_source_code (& source) ; unwrap_or_emit_fatal (source_str_to_stream (psess , filename , source , Some (item . span))) }
}

macro_rules! fake_token_stream_for_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fake_token_stream_for_crate in module {}", module_path!());
    };
}

mkfn!{
    fake_token_stream_for_crate_introspect!();
    pub fn fake_token_stream_for_crate (psess : & ParseSess , krate : & ast :: Crate) -> TokenStream { let source = pprust :: crate_to_string_for_macros (krate) ; let filename = FileName :: macro_expansion_source_code (& source) ; unwrap_or_emit_fatal (source_str_to_stream (psess , filename , source , Some (krate . spans . inner_span) ,)) }
}

macro_rules! parse_cfg_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_cfg_attr in module {}", module_path!());
    };
}

mkfn!{
    parse_cfg_attr_introspect!();
    pub fn parse_cfg_attr (cfg_attr : & Attribute , psess : & ParseSess ,) -> Option < (MetaItemInner , Vec < (AttrItem , Span) >) > { const CFG_ATTR_GRAMMAR_HELP : & str = "#[cfg_attr(condition, attribute, other_attribute, ...)]" ; const CFG_ATTR_NOTE_REF : & str = "for more information, visit \
        <https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute>" ; match cfg_attr . get_normal_item () . args { ast :: AttrArgs :: Delimited (ast :: DelimArgs { dspan , delim , ref tokens }) if ! tokens . is_empty () => { check_cfg_attr_bad_delim (psess , dspan , delim) ; match parse_in (psess , tokens . clone () , "`cfg_attr` input" , | p | p . parse_cfg_attr ()) { Ok (r) => return Some (r) , Err (e) => { e . with_help (format ! ("the valid syntax is `{CFG_ATTR_GRAMMAR_HELP}`")) . with_note (CFG_ATTR_NOTE_REF) . emit () ; } } } _ => { psess . dcx () . emit_err (errors :: MalformedCfgAttr { span : cfg_attr . span , sugg : CFG_ATTR_GRAMMAR_HELP , }) ; } } None }
}

macro_rules! check_cfg_attr_bad_delim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_cfg_attr_bad_delim in module {}", module_path!());
    };
}

mkfn!{
    check_cfg_attr_bad_delim_introspect!();
    fn check_cfg_attr_bad_delim (psess : & ParseSess , span : DelimSpan , delim : Delimiter) { if let Delimiter :: Parenthesis = delim { return ; } psess . dcx () . emit_err (errors :: CfgAttrBadDelim { span : span . entire () , sugg : errors :: MetaBadDelimSugg { open : span . open , close : span . close } , }) ; }
}