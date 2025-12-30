// Generated macro for impl_638 (impl)
macro_rules! Depcrate_utilsimpl_638 {
() => {
// Module: crate::utils
// Provides: {"impl_638"}
// Dependencies: {}
impl < 'i , 's , B > Deref for CowRef < 'i , 's , B > where B : ToOwned + ? Sized , B :: Owned : Borrow < B > , { type Target = B ; fn deref (& self) -> & B { match * self { Self :: Input (borrowed) => borrowed , Self :: Slice (borrowed) => borrowed , Self :: Owned (ref owned) => owned . borrow () , } } }
};
}
