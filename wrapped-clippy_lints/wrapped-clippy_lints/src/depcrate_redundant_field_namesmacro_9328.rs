// Generated macro for macro_9328 (macro)
macro_rules! Depcrate_redundant_field_namesmacro_9328 {
() => {
// Module: crate::redundant_field_names
// Provides: {"macro_9328"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for fields in struct literals where shorthands"] # [doc = " could be used."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If the field and variable names are the same,"] # [doc = " the field name is redundant."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let bar: u8 = 123;"] # [doc = ""] # [doc = " struct Foo {"] # [doc = "     bar: u8,"] # [doc = " }"] # [doc = ""] # [doc = " let foo = Foo { bar: bar };"] # [doc = " ```"] # [doc = " the last line can be simplified to"] # [doc = " ```ignore"] # [doc = " let foo = Foo { bar };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub REDUNDANT_FIELD_NAMES , style , "checks for fields in struct literals where shorthands could be used" }
};
}
