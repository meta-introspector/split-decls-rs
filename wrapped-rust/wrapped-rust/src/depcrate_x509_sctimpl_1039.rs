// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_x509_sctimpl_1039 {
() => {
// Module: crate::x509::sct
// Provides: {"impl_1039"}
// Dependencies: {}
impl < 'a > TLSReader < 'a > { fn new (data : & 'a [u8]) -> TLSReader < 'a > { TLSReader { data } } fn is_empty (& self) -> bool { self . data . is_empty () } fn read_byte (& mut self) -> Result < u8 , CryptographyError > { Ok (self . read_exact (1) ? [0]) } fn read_exact (& mut self , length : usize) -> Result < & 'a [u8] , CryptographyError > { if length > self . data . len () { return Err (CryptographyError :: from (pyo3 :: exceptions :: PyValueError :: new_err ("Invalid SCT length") ,)) ; } let (result , data) = self . data . split_at (length) ; self . data = data ; Ok (result) } fn read_length_prefixed (& mut self) -> Result < TLSReader < 'a > , CryptographyError > { let length = u16 :: from_be_bytes (self . read_exact (2) ? . try_into () . unwrap ()) ; Ok (TLSReader :: new (self . read_exact (length . into ()) ?)) } }
};
}
