// Generated macro for macro_10838 (macro)
macro_rules! Depcrate_unit_typesmacro_10838 {
() => {
// Module: crate::unit_types
// Provides: {"macro_10838"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for comparisons to unit. This includes all binary"] # [doc = " comparisons (like `==` and `<`) and asserts."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unit is always equal to itself, and thus is just a"] # [doc = " clumsily written constant. Mostly this happens when someone accidentally"] # [doc = " adds semicolons at the end of the operands."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn foo() {};"] # [doc = " # fn bar() {};"] # [doc = " # fn baz() {};"] # [doc = " if {"] # [doc = "     foo();"] # [doc = " } == {"] # [doc = "     bar();"] # [doc = " } {"] # [doc = "     baz();"] # [doc = " }"] # [doc = " ```"] # [doc = " is equal to"] # [doc = " ```no_run"] # [doc = " # fn foo() {};"] # [doc = " # fn bar() {};"] # [doc = " # fn baz() {};"] # [doc = " {"] # [doc = "     foo();"] # [doc = "     bar();"] # [doc = "     baz();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " For asserts:"] # [doc = " ```no_run"] # [doc = " # fn foo() {};"] # [doc = " # fn bar() {};"] # [doc = " assert_eq!({ foo(); }, { bar(); });"] # [doc = " ```"] # [doc = " will always succeed"] # [clippy :: version = "pre 1.29.0"] pub UNIT_CMP , correctness , "comparing unit values" }
};
}
