// Generated macro for StructuredInput (struct)
macro_rules! Depcrate_parseStructuredInput {
() => {
// Module: crate::parse
// Provides: {"StructuredInput"}
// Dependencies: {}
# [doc = " The input provided to our proc macro, after parsing into the form we expect."] # [derive (Debug)] pub struct StructuredInput { # [doc = " Macro to invoke once per function"] pub callback : Ident , # [doc = " Whether or not to provide `CFn` `CArgs` `RustFn` etc. This is really only needed"] # [doc = " once for crate to set up the main trait."] pub emit_types : Vec < Ident > , # [doc = " Skip these functions"] pub skip : Vec < Ident > , # [doc = " If true, omit f16 and f128 functions that aren't present in other libraries."] pub skip_f16_f128 : bool , # [doc = " Invoke only for these functions"] pub only : Option < Vec < Ident > > , # [doc = " Attributes that get applied to specific functions"] pub attributes : Option < Vec < AttributeMap > > , # [doc = " Extra expressions to pass to all invocations of the macro"] pub extra : Option < Expr > , # [doc = " Per-function extra expressions to pass to the macro"] pub fn_extra : Option < BTreeMap < Ident , Expr > > , pub emit_types_span : Option < Span > , pub only_span : Option < Span > , pub fn_extra_span : Option < Span > , }
};
}
