// Generated macro for impl_1235 (impl)
macro_rules! Depcrate_sslimpl_1235 {
() => {
// Module: crate::ssl
// Provides: {"impl_1235"}
// Dependencies: {}
# [allow (deprecated)] impl < S > SslStreamBuilder < S > { # [doc = " Returns a shared reference to the underlying stream."] pub fn get_ref (& self) -> & S { unsafe { let bio = self . inner . ssl . get_raw_rbio () ; bio :: get_ref (bio) } } # [doc = " Returns a mutable reference to the underlying stream."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " It is inadvisable to read from or write to the underlying stream as it"] # [doc = " will most likely corrupt the SSL session."] pub fn get_mut (& mut self) -> & mut S { unsafe { let bio = self . inner . ssl . get_raw_rbio () ; bio :: get_mut (bio) } } # [doc = " Returns a shared reference to the `Ssl` object associated with this builder."] pub fn ssl (& self) -> & SslRef { & self . inner . ssl } # [doc = " Set the DTLS MTU size."] # [doc = ""] # [doc = " It will be ignored if the value is smaller than the minimum packet size"] # [doc = " the DTLS protocol requires."] # [doc = ""] # [doc = " # Panics"] # [doc = " This function panics if the given mtu size can't be represented in a positive `c_long` range"] # [deprecated (note = "Use SslRef::set_mtu instead" , since = "0.10.30")] pub fn set_dtls_mtu_size (& mut self , mtu_size : usize) { unsafe { let bio = self . inner . ssl . get_raw_rbio () ; bio :: set_dtls_mtu_size :: < S > (bio , mtu_size) ; } } }
};
}
