// Generated macro for impl_136 (impl)
macro_rules! Depcrate_fullimpl_136 {
() => {
// Module: crate::full
// Provides: {"impl_136"}
// Dependencies: {}
impl < D , B > From < Cow < 'static , B > > for Full < D > where D : Buf + From < & 'static B > + From < B :: Owned > , B : ToOwned + ? Sized , { fn from (cow : Cow < 'static , B >) -> Self { match cow { Cow :: Borrowed (b) => Full :: new (D :: from (b)) , Cow :: Owned (o) => Full :: new (D :: from (o)) , } } }
};
}
