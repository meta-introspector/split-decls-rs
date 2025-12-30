// Generated macro for impl_96 (impl)
macro_rules! Depcrate_decodeimpl_96 {
() => {
// Module: crate::decode
// Provides: {"impl_96"}
// Dependencies: {}
# [doc = " Dummy implementation for [`PhantomData`] which allows deriving"] # [doc = " implementations on structs with phantom fields."] impl < 'a , T > Decode < 'a > for PhantomData < T > where T : ? Sized + 'a , { type Error = Error ; fn decode < R : Reader < 'a > > (_reader : & mut R) -> Result < PhantomData < T > , Error > { Ok (PhantomData) } }
};
}
