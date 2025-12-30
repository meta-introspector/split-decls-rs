// Generated macro for impl_902 (impl)
macro_rules! Depcrate_utf8impl_902 {
() => {
// Module: crate::utf8
// Provides: {"impl_902"}
// Dependencies: {}
impl fmt :: Debug for Utf8Sequence { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use self :: Utf8Sequence :: * ; match * self { One (ref r) => write ! (f , "{r:?}") , Two (ref r) => write ! (f , "{:?}{:?}" , r [0] , r [1]) , Three (ref r) => write ! (f , "{:?}{:?}{:?}" , r [0] , r [1] , r [2]) , Four (ref r) => { write ! (f , "{:?}{:?}{:?}{:?}" , r [0] , r [1] , r [2] , r [3]) } } } }
};
}
