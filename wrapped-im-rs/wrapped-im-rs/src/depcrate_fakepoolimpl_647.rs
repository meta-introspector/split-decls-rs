// Generated macro for impl_647 (impl)
macro_rules! Depcrate_fakepoolimpl_647 {
() => {
// Module: crate::fakepool
// Provides: {"impl_647"}
// Dependencies: {}
impl < A > Rc < A > { # [inline (always)] pub (crate) fn default (_pool : & Pool < A >) -> Self where A : PoolDefault , { Self (Default :: default ()) } # [inline (always)] pub (crate) fn new (_pool : & Pool < A > , value : A) -> Self { Rc (RRc :: new (value)) } # [inline (always)] pub (crate) fn clone_from (_pool : & Pool < A > , value : & A) -> Self where A : PoolClone , { Rc (RRc :: new (value . clone ())) } # [inline (always)] pub (crate) fn make_mut < 'a > (_pool : & Pool < A > , this : & 'a mut Self) -> & 'a mut A where A : PoolClone , { RRc :: make_mut (& mut this . 0) } # [inline (always)] pub (crate) fn ptr_eq (left : & Self , right : & Self) -> bool { RRc :: ptr_eq (& left . 0 , & right . 0) } pub (crate) fn unwrap_or_clone (this : Self) -> A where A : PoolClone , { RRc :: try_unwrap (this . 0) . unwrap_or_else (| r | (* r) . clone ()) } }
};
}
