// Generated macro for impl_639 (impl)
macro_rules! Depcrate_utilsimpl_639 {
() => {
// Module: crate::utils
// Provides: {"impl_639"}
// Dependencies: {}
impl < 'i , 's , B > Debug for CowRef < 'i , 's , B > where B : ToOwned + ? Sized + Debug , B :: Owned : Debug , { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match * self { Self :: Input (borrowed) => Debug :: fmt (borrowed , f) , Self :: Slice (borrowed) => Debug :: fmt (borrowed , f) , Self :: Owned (ref owned) => Debug :: fmt (owned , f) , } } }
};
}
