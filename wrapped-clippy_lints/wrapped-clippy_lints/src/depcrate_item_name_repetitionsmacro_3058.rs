// Generated macro for macro_3058 (macro)
macro_rules! Depcrate_item_name_repetitionsmacro_3058 {
() => {
// Module: crate::item_name_repetitions
// Provides: {"macro_3058"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for modules that have the same name as their"] # [doc = " parent module"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " A typical beginner mistake is to have `mod foo;` and"] # [doc = " again `mod foo { .."] # [doc = " }` in `foo.rs`."] # [doc = " The expectation is that items inside the inner `mod foo { .. }` are then"] # [doc = " available"] # [doc = " through `foo::x`, but they are only available through"] # [doc = " `foo::foo::x`."] # [doc = " If this is done on purpose, it would be better to choose a more"] # [doc = " representative module name."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " // lib.rs"] # [doc = " mod foo;"] # [doc = " // foo.rs"] # [doc = " mod foo {"] # [doc = "     ..."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MODULE_INCEPTION , style , "modules that have the same name as their parent module" }
};
}
