// Generated macro for impl_113 (impl)
macro_rules! Depcrate_encodeimpl_113 {
() => {
// Module: crate::encode
// Provides: {"impl_113"}
// Dependencies: {}
# [doc = " Dummy implementation for [`PhantomData`] which allows deriving"] # [doc = " implementations on structs with phantom fields."] impl < T > Encode for PhantomData < T > where T : ? Sized , { fn encoded_len (& self) -> Result < Length > { Ok (Length :: ZERO) } fn encode (& self , _writer : & mut impl Writer) -> Result < () > { Ok (()) } }
};
}
