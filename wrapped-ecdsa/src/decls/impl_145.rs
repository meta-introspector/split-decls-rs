macro_rules! deps {
    () => {
        MaxOverhead!();
        MaxSize!();
        EcdsaCurve!();
        SignatureSize!();
        SignatureBytes!();
        Signature!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < C > Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { # [doc = " Parse a signature from fixed-width bytes, i.e. 2 * the size of"] # [doc = " [`FieldBytes`] for a particular curve."] # [doc = ""] # [doc = " # Returns"] # [doc = " - `Ok(signature)` if the `r` and `s` components are both in the valid"] # [doc = "   range `1..n` when serialized as concatenated big endian integers."] # [doc = " - `Err(err)` if the `r` and/or `s` component of the signature is"] # [doc = "   out-of-range when interpreted as a big endian integer."] pub fn from_bytes (bytes : & SignatureBytes < C >) -> Result < Self > { let chunks = FieldBytes :: < C > :: slice_as_chunks (bytes) . 0 ; let r = chunks [0] . clone () ; let s = chunks [1] . clone () ; Self :: from_scalars (r , s) } # [doc = " Parse a signature from a byte slice."] pub fn from_slice (slice : & [u8]) -> Result < Self > { < & SignatureBytes < C > > :: try_from (slice) . map_err (| _ | Error :: new ()) . and_then (Self :: from_bytes) } # [doc = " Parse a signature from ASN.1 DER."] # [cfg (feature = "der")] pub fn from_der (bytes : & [u8]) -> Result < Self > where der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { der :: Signature :: < C > :: try_from (bytes) . and_then (Self :: try_from) } # [doc = " Create a [`Signature`] from the serialized `r` and `s` scalar values"] # [doc = " which comprise the signature."] # [doc = ""] # [doc = " # Returns"] # [doc = " - `Ok(signature)` if the `r` and `s` components are both in the valid"] # [doc = "   range `1..n` when serialized as concatenated big endian integers."] # [doc = " - `Err(err)` if the `r` and/or `s` component of the signature is"] # [doc = "   out-of-range when interpreted as a big endian integer."] pub fn from_scalars (r : impl Into < FieldBytes < C > > , s : impl Into < FieldBytes < C > >) -> Result < Self > { let r = ScalarValue :: from_slice (& r . into ()) . map_err (| _ | Error :: new ()) ? ; let s = ScalarValue :: from_slice (& s . into ()) . map_err (| _ | Error :: new ()) ? ; if r . is_zero () . into () || s . is_zero () . into () { return Err (Error :: new ()) ; } Ok (Self { r , s }) } # [doc = " Split the signature into its `r` and `s` components, represented as bytes."] pub fn split_bytes (& self) -> (FieldBytes < C > , FieldBytes < C >) { (self . r . to_bytes () , self . s . to_bytes ()) } # [doc = " Serialize this signature as bytes."] pub fn to_bytes (& self) -> SignatureBytes < C > { let mut bytes = SignatureBytes :: < C > :: default () ; let (r_bytes , s_bytes) = bytes . split_at_mut (C :: FieldBytesSize :: USIZE) ; r_bytes . copy_from_slice (& self . r . to_bytes ()) ; s_bytes . copy_from_slice (& self . s . to_bytes ()) ; bytes } # [doc = " Serialize this signature as ASN.1 DER."] # [cfg (feature = "der")] pub fn to_der (& self) -> der :: Signature < C > where der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { let (r , s) = self . split_bytes () ; der :: Signature :: from_components (& r , & s) . expect ("DER encoding error") } # [doc = " Convert this signature into a byte vector."] # [cfg (feature = "alloc")] pub fn to_vec (& self) -> Vec < u8 > { self . to_bytes () . to_vec () } }
    };
}

impl_145!()