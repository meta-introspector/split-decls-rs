// Generated macro for PathTransform (struct)
macro_rules! Depcrate_path_transformPathTransform {
() => {
// Module: crate::path_transform
// Provides: {"PathTransform"}
// Dependencies: {}
# [doc = " `PathTransform` substitutes path in SyntaxNodes in bulk."] # [doc = ""] # [doc = " This is mostly useful for IDE code generation. If you paste some existing"] # [doc = " code into a new context (for example, to add method overrides to an `impl`"] # [doc = " block), you generally want to appropriately qualify the names, and sometimes"] # [doc = " you might want to substitute generic parameters as well:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " mod x {"] # [doc = "   pub struct A<V>;"] # [doc = "   pub trait T<U> { fn foo(&self, _: U) -> A<U>; }"] # [doc = " }"] # [doc = ""] # [doc = " mod y {"] # [doc = "   use x::T;"] # [doc = ""] # [doc = "   impl T<()> for () {"] # [doc = "      // If we invoke **Add Missing Members** here, we want to copy-paste `foo`."] # [doc = "      // But we want a slightly-modified version of it:"] # [doc = "      fn foo(&self, _: ()) -> x::A<()> {}"] # [doc = "   }"] # [doc = " }"] # [doc = " ```"] pub struct PathTransform < 'a > { generic_def : Option < hir :: GenericDef > , substs : AstSubsts , target_scope : & 'a SemanticsScope < 'a > , source_scope : & 'a SemanticsScope < 'a > , }
};
}
