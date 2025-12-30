// Generated macro for impl_79 (impl)
macro_rules! Depcrate_decodeimpl_79 {
() => {
// Module: crate::decode
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'de , R > Deserializer < ReadRefReader < 'de , R > > where R : AsRef < [u8] > + ? Sized , { # [doc = " Constructs a new `Deserializer` from the given byte slice."] # [inline (always)] pub fn from_read_ref (rd : & 'de R) -> Self { Deserializer { rd : ReadRefReader :: new (rd) , is_human_readable : DefaultConfig . is_human_readable () , _config : PhantomData , marker : None , depth : 1024 , } } # [doc = " Gets a reference to the underlying reader in this decoder."] # [inline (always)] # [must_use] pub fn get_ref (& self) -> & R { self . rd . whole_slice } }
};
}
