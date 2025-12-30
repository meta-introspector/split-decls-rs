// Generated macro for impl_9346 (impl)
macro_rules! Depcrate_returnsimpl_9346 {
() => {
// Module: crate::returns
// Provides: {"impl_9346"}
// Dependencies: {}
impl Display for RetReplacement < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: Empty => write ! (f , "") , Self :: Block => write ! (f , "{{}}") , Self :: Unit => write ! (f , "()") , Self :: NeedsPar (inner , _) => write ! (f , "({inner})") , Self :: Expr (inner , _) => write ! (f , "{inner}") , } } }
};
}
