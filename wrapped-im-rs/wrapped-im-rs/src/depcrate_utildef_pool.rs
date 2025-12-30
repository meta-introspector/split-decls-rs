// Generated macro for def_pool (macro)
macro_rules! Depcrate_utildef_pool {
() => {
// Module: crate::util
// Provides: {"def_pool"}
// Dependencies: {}
macro_rules ! def_pool { ($ name : ident <$ ($ arg : tt) ,*>, $ pooltype : ty) => { # [doc = " A memory pool for the appropriate node type."] pub struct $ name <$ ($ arg ,) *> (Pool <$ pooltype >) ; impl <$ ($ arg ,) *> $ name <$ ($ arg ,) *> { # [doc = " Create a new pool with the given size."] pub fn new (size : usize) -> Self { Self (Pool :: new (size)) } # [doc = " Fill the pool with preallocated chunks."] pub fn fill (& self) { self . 0 . fill () ; } # [doc = "Get the current size of the pool."] pub fn pool_size (& self) -> usize { self . 0 . get_pool_size () } } impl <$ ($ arg ,) *> Default for $ name <$ ($ arg ,) *> { fn default () -> Self { Self :: new ($ crate :: config :: POOL_SIZE) } } impl <$ ($ arg ,) *> Clone for $ name <$ ($ arg ,) *> { fn clone (& self) -> Self { Self (self . 0 . clone ()) } } } ; }
};
}
