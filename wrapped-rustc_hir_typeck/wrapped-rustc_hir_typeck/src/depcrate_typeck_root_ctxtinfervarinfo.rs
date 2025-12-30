// Generated macro for InferVarInfo (struct)
macro_rules! Depcrate_typeck_root_ctxtInferVarInfo {
() => {
// Module: crate::typeck_root_ctxt
// Provides: {"InferVarInfo"}
// Dependencies: {}
# [derive (Debug , Default , Copy , Clone)] pub (crate) struct InferVarInfo { # [doc = " This is true if we identified that this Ty (`?T`) is found in a `?T: Foo`"] # [doc = " obligation, where:"] # [doc = ""] # [doc = "  * `Foo` is not `Sized`"] # [doc = "  * `(): Foo` may be satisfied"] pub self_in_trait : bool , # [doc = " This is true if we identified that this Ty (`?T`) is found in a `<_ as"] # [doc = " _>::AssocType = ?T`"] pub output : bool , }
};
}
