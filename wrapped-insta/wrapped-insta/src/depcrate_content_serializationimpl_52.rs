// Generated macro for impl_52 (impl)
macro_rules! Depcrate_content_serializationimpl_52 {
() => {
// Module: crate::content::serialization
// Provides: {"impl_52"}
// Dependencies: {}
impl Key < '_ > { # [doc = " Needed because [`std::mem::discriminant`] is not [`Ord`]"] fn discriminant (& self) -> usize { match self { Key :: Bool (_) => 1 , Key :: U64 (_) => 2 , Key :: I64 (_) => 3 , Key :: F64 (_) => 4 , Key :: U128 (_) => 5 , Key :: I128 (_) => 6 , Key :: Str (_) => 7 , Key :: Bytes (_) => 8 , Key :: Other => 9 , } } }
};
}
