// Generated macro for impl_312 (impl)
macro_rules! Depcrate_errorimpl_312 {
() => {
// Module: crate::error
// Provides: {"impl_312"}
// Dependencies: {}
# [doc = " Add child diagnostics to the error."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ## Code"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use darling_core::Error;"] # [doc = " # let struct_ident = proc_macro2::Span::call_site();"] # [doc = " Error::custom(\"this is a demo\")"] # [doc = "     .with_span(&struct_ident)"] # [doc = "     .note(\"we wrote this\")"] # [doc = "     .help(\"try doing this instead\");"] # [doc = " ```"] # [doc = " ## Output"] # [doc = ""] # [doc = " ```text"] # [doc = " error: this is a demo"] # [doc = "   --> my_project/my_file.rs:3:5"] # [doc = "    |"] # [doc = " 13 |     FooBar { value: String },"] # [doc = "    |     ^^^^^^"] # [doc = "    |"] # [doc = "    = note: we wrote this"] # [doc = "    = help: try doing this instead"] # [doc = " ```"] # [cfg (feature = "diagnostics")] impl Error { add_child ! (error , span_error , Error) ; add_child ! (warning , span_warning , Warning) ; add_child ! (note , span_note , Note) ; add_child ! (help , span_help , Help) ; }
};
}
