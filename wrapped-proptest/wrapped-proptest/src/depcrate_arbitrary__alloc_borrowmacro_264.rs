// Generated macro for macro_264 (macro)
macro_rules! Depcrate_arbitrary__alloc_borrowmacro_264 {
() => {
// Module: crate::arbitrary::_alloc::borrow
// Provides: {"macro_264"}
// Dependencies: {}
arbitrary ! ([A : Arbitrary + Borrow < B >, B : ToOwned < Owned = A > + fmt :: Debug + ? Sized] Cow <'static , B >, SMapped < A , Self >, A :: Parameters ; args => static_map (any_with ::< A > (args) , Cow :: Owned)) ;
};
}
