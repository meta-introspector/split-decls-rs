// Generated macro for CtxHash (trait)
macro_rules! Depcrate_ctxhashCtxHash {
() => {
// Module: crate::ctxhash
// Provides: {"CtxHash"}
// Dependencies: {}
# [doc = " Trait that allows for hashing given some external context."] pub trait CtxHash < Value : ? Sized > : CtxEq < Value , Value > { # [doc = " Compute the hash of `value`, given the context in `self` and"] # [doc = " the union-find data structure `uf`."] fn ctx_hash < H : Hasher > (& self , state : & mut H , value : & Value) ; }
};
}
