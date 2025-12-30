// Generated macro for FamousDefs (struct)
macro_rules! Depcrate_famous_defsFamousDefs {
() => {
// Module: crate::famous_defs
// Provides: {"FamousDefs"}
// Dependencies: {}
# [doc = " Helps with finding well-know things inside the standard library. This is"] # [doc = " somewhat similar to the known paths infra inside hir, but it different; We"] # [doc = " want to make sure that IDE specific paths don't become interesting inside"] # [doc = " the compiler itself as well."] # [doc = ""] # [doc = " Note that, by default, rust-analyzer tests **do not** include core or std"] # [doc = " libraries. If you are writing tests for functionality using [`FamousDefs`],"] # [doc = " you'd want to include minicore (see `test_utils::MiniCore`) declaration at"] # [doc = " the start of your tests:"] # [doc = ""] # [doc = " ```text"] # [doc = " //- minicore: iterator, ord, derive"] # [doc = " ```"] pub struct FamousDefs < 'a , 'b > (pub & 'a Semantics < 'b , RootDatabase > , pub Crate) ;
};
}
