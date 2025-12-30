// Generated macro for impl_46 (impl)
macro_rules! Depcrate_popimpl_46 {
() => {
// Module: crate::pop
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'a > :: der :: Decode < 'a > for EncKeyWithIdChoice < 'a > { type Error = :: der :: Error ; fn decode < R : :: der :: Reader < 'a > > (reader : & mut R) -> :: der :: Result < Self > { let t = der :: Tag :: peek (reader) ? ; if t == < Utf8StringRef < 'a > as :: der :: FixedTag > :: TAG { Ok (Self :: String (reader . decode () ?)) } else if t . is_context_specific () { Ok (Self :: GeneralName (reader . decode () ?)) } else { Err (der :: ErrorKind :: TagUnexpected { expected : None , actual : t , } . into ()) } } }
};
}
