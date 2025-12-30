// Generated macro for nth_field_name (function)
macro_rules! Depcrate_property_test_codegennth_field_name {
() => {
// Module: crate::property_test::codegen
// Provides: {"nth_field_name"}
// Dependencies: {}
# [doc = " The rule for field names is:"] # [doc = " - if the arguments pattern is an ident, we reuse that ident verbatim"] # [doc = " - otherwise, we use the name `arg<n>`, where `<n>` is the index of the argument (including"] # [doc = " ident arguments)"] # [doc = ""] # [doc = " So for example, given the args `foo: i32, (a, b): (i32, bool), baz: bool`, the generated struct"] # [doc = " would roughly be:"] # [doc = " ```rust"] # [doc = " struct Args {"] # [doc = "     foo: i32,"] # [doc = "     arg1: (i32, bool),"] # [doc = "     baz: bool,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Panics if `index` is out of bounds for `args`"] fn nth_field_name (args : & [Argument] , index : usize) -> Ident { let arg = & args [index] ; match arg . pat_ty . pat . as_ref () { Pat :: Ident (pat_ident) => pat_ident . ident . clone () , other => Ident :: new (& format ! ("arg{index}") , other . span ()) , } }
};
}
