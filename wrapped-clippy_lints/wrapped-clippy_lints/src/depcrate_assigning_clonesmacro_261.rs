// Generated macro for macro_261 (macro)
macro_rules! Depcrate_assigning_clonesmacro_261 {
() => {
// Module: crate::assigning_clones
// Provides: {"macro_261"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for code like `foo = bar.clone();`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Custom `Clone::clone_from()` or `ToOwned::clone_into` implementations allow the objects"] # [doc = " to share resources and therefore avoid allocations."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " struct Thing;"] # [doc = ""] # [doc = " impl Clone for Thing {"] # [doc = "     fn clone(&self) -> Self { todo!() }"] # [doc = "     fn clone_from(&mut self, other: &Self) { todo!() }"] # [doc = " }"] # [doc = ""] # [doc = " pub fn assign_to_ref(a: &mut Thing, b: Thing) {"] # [doc = "     *a = b.clone();"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " struct Thing;"] # [doc = ""] # [doc = " impl Clone for Thing {"] # [doc = "     fn clone(&self) -> Self { todo!() }"] # [doc = "     fn clone_from(&mut self, other: &Self) { todo!() }"] # [doc = " }"] # [doc = ""] # [doc = " pub fn assign_to_ref(a: &mut Thing, b: Thing) {"] # [doc = "     a.clone_from(&b);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub ASSIGNING_CLONES , pedantic , "assigning the result of cloning may be inefficient" }
};
}
