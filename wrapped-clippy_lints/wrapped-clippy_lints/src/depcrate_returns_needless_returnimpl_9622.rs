// Generated macro for impl_9622 (impl)
macro_rules! Depcrate_returns_needless_returnimpl_9622 {
() => {
// Module: crate::returns::needless_return
// Provides: {"impl_9622"}
// Dependencies: {}
impl Display for RetReplacement < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Empty => f . write_str ("") , Self :: Block => f . write_str ("{}") , Self :: Unit => f . write_str ("()") , Self :: NeedsPar (inner , _) => write ! (f , "({inner})") , Self :: Expr (inner , _) => write ! (f , "{inner}") , } } }
};
}
