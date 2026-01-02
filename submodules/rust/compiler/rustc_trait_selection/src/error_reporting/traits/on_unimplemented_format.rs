mkuse!{use std :: fmt ;}
mkuse!{use std :: ops :: Range ;}
mkuse!{use errors :: * ;}
mkuse!{use rustc_middle :: ty :: print :: TraitRefPrintSugared ;}
mkuse!{use rustc_middle :: ty :: { GenericParamDefKind , TyCtxt } ;}
mkuse!{use rustc_parse_format :: { Argument , FormatSpec , ParseError , ParseMode , Parser , Piece as RpfPiece , Position , } ;}
mkuse!{use rustc_session :: lint :: builtin :: MALFORMED_DIAGNOSTIC_FORMAT_LITERALS ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use rustc_span :: { InnerSpan , Span , Symbol , kw , sym } ;}
mkitem!{mkstruct!{# [doc = " Like [std::fmt::Arguments] this is a string that has been parsed into \"pieces\","] # [doc = " either as string pieces or dynamic arguments."] # [derive (Debug)] pub struct FormatString { # [allow (dead_code , reason = "Debug impl")] input : Symbol , span : Span , pieces : Vec < Piece > , # [doc = " The formatting string was parsed successfully but with warnings"] pub warnings : Vec < FormatWarning > , }}}
mkitem!{mkenum!{# [derive (Debug)] enum Piece { Lit (String) , Arg (FormatArg) , }}}
mkitem!{mkenum!{# [derive (Debug)] enum FormatArg { GenericParam { generic_param : Symbol , } , SelfUpper , # [doc = " `{This}` or `{TraitName}`"] This , # [doc = " The sugared form of the trait"] Trait , # [doc = " what we're in, like a function, method, closure etc."] ItemContext , # [doc = " What the user typed, if it doesn't match anything we can use."] AsIs (String) , }}}
mkitem!{mkenum!{pub enum Ctx < 'tcx > { RustcOnUnimplemented { tcx : TyCtxt < 'tcx > , trait_def_id : DefId } , DiagnosticOnUnimplemented { tcx : TyCtxt < 'tcx > , trait_def_id : DefId } , }}}
mkitem!{mkenum!{# [derive (Debug)] pub enum FormatWarning { UnknownParam { argument_name : Symbol , span : Span } , PositionalArgument { span : Span , help : String } , InvalidSpecifier { name : String , span : Span } , FutureIncompat { span : Span , help : String } , }}}
mkitem!{mkimpl!{impl FormatWarning { pub fn emit_warning < 'tcx > (& self , tcx : TyCtxt < 'tcx > , item_def_id : DefId) { match * self { FormatWarning :: UnknownParam { argument_name , span } => { let this = tcx . item_ident (item_def_id) ; if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , tcx . local_def_id_to_hir_id (item_def_id) , span , UnknownFormatParameterForOnUnimplementedAttr { argument_name , trait_name : this , } ,) ; } } FormatWarning :: PositionalArgument { span , .. } => { if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , tcx . local_def_id_to_hir_id (item_def_id) , span , DisallowedPositionalArgument ,) ; } } FormatWarning :: InvalidSpecifier { span , .. } => { if let Some (item_def_id) = item_def_id . as_local () { tcx . emit_node_span_lint (MALFORMED_DIAGNOSTIC_FORMAT_LITERALS , tcx . local_def_id_to_hir_id (item_def_id) , span , InvalidFormatSpecifier ,) ; } } FormatWarning :: FutureIncompat { .. } => { } } } }}}
mkitem!{mkstruct!{# [doc = " Arguments to fill a [FormatString] with."] # [doc = ""] # [doc = " For example, given a"] # [doc = " ```rust,ignore (just an example)"] # [doc = ""] # [doc = " #[rustc_on_unimplemented("] # [doc = "     on(all(from_desugaring = \"QuestionMark\"),"] # [doc = "         message = \"the `?` operator can only be used in {ItemContext} \\"] # [doc = "                     that returns `Result` or `Option` \\"] # [doc = "                     (or another type that implements `{FromResidual}`)\","] # [doc = "         label = \"cannot use the `?` operator in {ItemContext} that returns `{Self}`\","] # [doc = "         parent_label = \"this function should return `Result` or `Option` to accept `?`\""] # [doc = "     ),"] # [doc = " )]"] # [doc = " pub trait FromResidual<R = <Self as Try>::Residual> {"] # [doc = "    ..."] # [doc = " }"] # [doc = ""] # [doc = " async fn an_async_function() -> u32 {"] # [doc = "     let x: Option<u32> = None;"] # [doc = "     x?; //~ ERROR the `?` operator"] # [doc = "     22"] # [doc = " }"] # [doc = "  ```"] # [doc = " it will look like this:"] # [doc = ""] # [doc = " ```rust,ignore (just an example)"] # [doc = " FormatArgs {"] # [doc = "     this: \"FromResidual\","] # [doc = "     trait_sugared: \"FromResidual<Option<Infallible>>\","] # [doc = "     item_context: \"an async function\","] # [doc = "     generic_args: [(\"Self\", \"u32\"), (\"R\", \"Option<Infallible>\")],"] # [doc = " }"] # [doc = " ```"] # [derive (Debug)] pub struct FormatArgs < 'tcx > { pub this : String , pub trait_sugared : TraitRefPrintSugared < 'tcx > , pub item_context : & 'static str , pub generic_args : Vec < (Symbol , String) > , }}}
mkitem!{mkimpl!{impl FormatString { pub fn span (& self) -> Span { self . span } pub fn parse < 'tcx > (input : Symbol , snippet : Option < String > , span : Span , ctx : & Ctx < 'tcx > ,) -> Result < Self , ParseError > { let s = input . as_str () ; let mut parser = Parser :: new (s , None , snippet , false , ParseMode :: Diagnostic) ; let pieces : Vec < _ > = parser . by_ref () . collect () ; if let Some (err) = parser . errors . into_iter () . next () { return Err (err) ; } let mut warnings = Vec :: new () ; let pieces = pieces . into_iter () . map (| piece | match piece { RpfPiece :: Lit (lit) => Piece :: Lit (lit . into ()) , RpfPiece :: NextArgument (arg) => { warn_on_format_spec (& arg . format , & mut warnings , span , parser . is_source_literal) ; let arg = parse_arg (& arg , ctx , & mut warnings , span , parser . is_source_literal) ; Piece :: Arg (arg) } }) . collect () ; Ok (FormatString { input , pieces , span , warnings }) } pub fn format (& self , args : & FormatArgs < '_ >) -> String { let mut ret = String :: new () ; for piece in & self . pieces { match piece { Piece :: Lit (s) | Piece :: Arg (FormatArg :: AsIs (s)) => ret . push_str (& s) , Piece :: Arg (FormatArg :: GenericParam { generic_param }) => { let value = match args . generic_args . iter () . find (| (p , _) | p == generic_param) { Some ((_ , val)) => val . to_string () , None => generic_param . to_string () , } ; ret . push_str (& value) ; } Piece :: Arg (FormatArg :: SelfUpper) => { let slf = match args . generic_args . iter () . find (| (p , _) | * p == kw :: SelfUpper) { Some ((_ , val)) => val . to_string () , None => "Self" . to_string () , } ; ret . push_str (& slf) ; } Piece :: Arg (FormatArg :: This) => ret . push_str (& args . this) , Piece :: Arg (FormatArg :: Trait) => { let _ = fmt :: write (& mut ret , format_args ! ("{}" , & args . trait_sugared)) ; } Piece :: Arg (FormatArg :: ItemContext) => ret . push_str (args . item_context) , } } ret } }}}

macro_rules! parse_arg_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_arg in module {}", module_path!());
    };
}

mkfn!{
    parse_arg_introspect!();
    fn parse_arg < 'tcx > (arg : & Argument < '_ > , ctx : & Ctx < 'tcx > , warnings : & mut Vec < FormatWarning > , input_span : Span , is_source_literal : bool ,) -> FormatArg { let (Ctx :: RustcOnUnimplemented { tcx , trait_def_id } | Ctx :: DiagnosticOnUnimplemented { tcx , trait_def_id }) = ctx ; let span = slice_span (input_span , arg . position_span . clone () , is_source_literal) ; match arg . position { Position :: ArgumentNamed (name) => match (ctx , Symbol :: intern (name)) { (Ctx :: RustcOnUnimplemented { .. } , sym :: ItemContext) => FormatArg :: ItemContext , (Ctx :: RustcOnUnimplemented { .. } , sym :: This) => FormatArg :: This , (Ctx :: RustcOnUnimplemented { .. } , sym :: Trait) => FormatArg :: Trait , (Ctx :: RustcOnUnimplemented { .. } | Ctx :: DiagnosticOnUnimplemented { .. } , kw :: SelfUpper ,) => FormatArg :: SelfUpper , (Ctx :: RustcOnUnimplemented { .. } | Ctx :: DiagnosticOnUnimplemented { .. } , generic_param ,) if tcx . generics_of (trait_def_id) . own_params . iter () . any (| param | { ! matches ! (param . kind , GenericParamDefKind :: Lifetime) && param . name == generic_param }) => { FormatArg :: GenericParam { generic_param } } (_ , argument_name) => { warnings . push (FormatWarning :: UnknownParam { argument_name , span }) ; FormatArg :: AsIs (format ! ("{{{}}}" , argument_name . as_str ())) } } , Position :: ArgumentIs (idx) => { warnings . push (FormatWarning :: PositionalArgument { span , help : format ! ("use `{{{idx}}}` to print a number in braces") , }) ; FormatArg :: AsIs (format ! ("{{{idx}}}")) } Position :: ArgumentImplicitlyIs (_) => { warnings . push (FormatWarning :: PositionalArgument { span , help : String :: from ("use `{{}}` to print empty braces") , }) ; FormatArg :: AsIs (String :: from ("{}")) } } }
}

macro_rules! warn_on_format_spec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function warn_on_format_spec in module {}", module_path!());
    };
}

mkfn!{
    warn_on_format_spec_introspect!();
    # [doc = " `#[rustc_on_unimplemented]` and `#[diagnostic::...]` don't actually do anything"] # [doc = " with specifiers, so emit a warning if they are used."] fn warn_on_format_spec (spec : & FormatSpec < '_ > , warnings : & mut Vec < FormatWarning > , input_span : Span , is_source_literal : bool ,) { if spec . ty != "" { let span = spec . ty_span . as_ref () . map (| inner | slice_span (input_span , inner . clone () , is_source_literal)) . unwrap_or (input_span) ; warnings . push (FormatWarning :: InvalidSpecifier { span , name : spec . ty . into () }) } }
}

macro_rules! slice_span_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function slice_span in module {}", module_path!());
    };
}

mkfn!{
    slice_span_introspect!();
    fn slice_span (input : Span , Range { start , end } : Range < usize > , is_source_literal : bool) -> Span { if is_source_literal { input . from_inner (InnerSpan { start , end }) } else { input } }
}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                mkuse!{use rustc_macros :: LintDiagnostic ;}
mkuse!{use rustc_span :: Ident ;}
mkuse!{use super :: * ;}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (trait_selection_unknown_format_parameter_for_on_unimplemented_attr)] # [help] pub struct UnknownFormatParameterForOnUnimplementedAttr { pub argument_name : Symbol , pub trait_name : Ident , }}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (trait_selection_disallowed_positional_argument)] # [help] pub struct DisallowedPositionalArgument ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (trait_selection_invalid_format_specifier)] # [help] pub struct InvalidFormatSpecifier ;}}
mkitem!{mkstruct!{# [derive (LintDiagnostic)] # [diag (trait_selection_missing_options_for_on_unimplemented_attr)] # [help] pub struct MissingOptionsForOnUnimplementedAttr ;}} 
            }}