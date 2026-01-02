mkuse!{use proc_macro :: { Diagnostic , Level , MultiSpan } ;}
mkuse!{use proc_macro2 :: TokenStream ;}
mkuse!{use quote :: quote ;}
mkuse!{use syn :: spanned :: Spanned ;}
mkuse!{use syn :: { Attribute , Error as SynError , Meta } ;}
mkitem!{mkenum!{# [derive (Debug)] pub (crate) enum DiagnosticDeriveError { SynError (SynError) , ErrorHandled , }}}
mkitem!{mkimpl!{impl DiagnosticDeriveError { pub (crate) fn to_compile_error (self) -> TokenStream { match self { DiagnosticDeriveError :: SynError (e) => e . to_compile_error () , DiagnosticDeriveError :: ErrorHandled => { quote ! { { unreachable ! () ; } } } } } }}}
mkitem!{mkimpl!{impl From < SynError > for DiagnosticDeriveError { fn from (e : SynError) -> Self { DiagnosticDeriveError :: SynError (e) } }}}

macro_rules! _throw_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _throw_err in module {}", module_path!());
    };
}

mkfn!{
    _throw_err_introspect!();
    # [doc = " Helper function for use with `throw_*` macros - constraints `$f` to an `impl FnOnce`."] pub (crate) fn _throw_err (diag : Diagnostic , f : impl FnOnce (Diagnostic) -> Diagnostic ,) -> DiagnosticDeriveError { f (diag) . emit () ; DiagnosticDeriveError :: ErrorHandled }
}

macro_rules! path_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_to_string in module {}", module_path!());
    };
}

mkfn!{
    path_to_string_introspect!();
    # [doc = " Helper function for printing `syn::Path` - doesn't handle arguments in paths and these are"] # [doc = " unlikely to come up much in use of the macro."] fn path_to_string (path : & syn :: Path) -> String { let mut out = String :: new () ; for (i , segment) in path . segments . iter () . enumerate () { if i > 0 || path . leading_colon . is_some () { out . push_str ("::") ; } out . push_str (& segment . ident . to_string ()) ; } out }
}

macro_rules! span_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function span_err in module {}", module_path!());
    };
}

mkfn!{
    span_err_introspect!();
    # [doc = " Returns an error diagnostic on span `span` with msg `msg`."] # [must_use] pub (crate) fn span_err < T : Into < String > > (span : impl MultiSpan , msg : T) -> Diagnostic { Diagnostic :: spanned (span , Level :: Error , format ! ("derive(Diagnostic): {}" , msg . into ())) }
}
mkitem!{# [doc = " Emit a diagnostic on span `$span` with msg `$msg` (optionally performing additional decoration"] # [doc = " using the `FnOnce` passed in `diag`) and return `Err(ErrorHandled)`."] # [doc = ""] # [doc = " For methods that return a `Result<_, DiagnosticDeriveError>`:"] macro_rules ! throw_span_err { ($ span : expr , $ msg : expr) => { { throw_span_err ! ($ span , $ msg , | diag | diag) } } ; ($ span : expr , $ msg : expr , $ f : expr) => { { let diag = span_err ($ span , $ msg) ; return Err (crate :: diagnostics :: error :: _throw_err (diag , $ f)) ; } } ; }}
mkuse!{pub (crate) use throw_span_err ;}

macro_rules! invalid_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function invalid_attr in module {}", module_path!());
    };
}

mkfn!{
    invalid_attr_introspect!();
    # [doc = " Returns an error diagnostic for an invalid attribute."] pub (crate) fn invalid_attr (attr : & Attribute) -> Diagnostic { let span = attr . span () . unwrap () ; let path = path_to_string (attr . path ()) ; match attr . meta { Meta :: Path (_) => span_err (span , format ! ("`#[{path}]` is not a valid attribute")) , Meta :: NameValue (_) => span_err (span , format ! ("`#[{path} = ...]` is not a valid attribute")) , Meta :: List (_) => span_err (span , format ! ("`#[{path}(...)]` is not a valid attribute")) , } }
}
mkitem!{# [doc = " Emit an error diagnostic for an invalid attribute (optionally performing additional decoration"] # [doc = " using the `FnOnce` passed in `diag`) and return `Err(ErrorHandled)`."] # [doc = ""] # [doc = " For methods that return a `Result<_, DiagnosticDeriveError>`:"] macro_rules ! throw_invalid_attr { ($ attr : expr) => { { throw_invalid_attr ! ($ attr , | diag | diag) } } ; ($ attr : expr , $ f : expr) => { { let diag = crate :: diagnostics :: error :: invalid_attr ($ attr) ; return Err (crate :: diagnostics :: error :: _throw_err (diag , $ f)) ; } } ; }}
mkuse!{pub (crate) use throw_invalid_attr ;}