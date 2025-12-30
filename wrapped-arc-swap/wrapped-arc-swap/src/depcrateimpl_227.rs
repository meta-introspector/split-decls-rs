// Generated macro for impl_227 (impl)
macro_rules! Depcrateimpl_227 {
() => {
// Module: crate
// Provides: {"impl_227"}
// Dependencies: {}
impl < T : RefCnt , S : Strategy < T > > Deref for Guard < T , S > { type Target = T ; # [inline] fn deref (& self) -> & T { self . inner . borrow () } }
};
}
