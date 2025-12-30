// Generated macro for CtxEq (trait)
macro_rules! Depcrate_ctxhashCtxEq {
() => {
// Module: crate::ctxhash
// Provides: {"CtxEq"}
// Dependencies: {}
# [doc = " Trait that allows for equality comparison given some external"] # [doc = " context."] # [doc = ""] # [doc = " Note that this trait is implemented by the *context*, rather than"] # [doc = " the item type, for somewhat complex lifetime reasons (lack of GATs"] # [doc = " to allow `for<'ctx> Ctx<'ctx>`-like associated types in traits on"] # [doc = " the value type)."] pub trait CtxEq < V1 : ? Sized , V2 : ? Sized > { # [doc = " Determine whether `a` and `b` are equal, given the context in"] # [doc = " `self` and the union-find data structure `uf`."] fn ctx_eq (& self , a : & V1 , b : & V2) -> bool ; }
};
}
