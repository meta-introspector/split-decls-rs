// Generated macro for impl_14 (impl)
macro_rules! Depcrate_hmac_implimpl_14 {
() => {
// Module: crate::hmac_impl
// Provides: {"impl_14"}
// Dependencies: {}
impl < H : EagerHash > HmacImpl for Hmac < H > { # [inline (always)] fn new_from_slice (key : & [u8]) -> Self { KeyInit :: new_from_slice (key) . expect ("HMAC can take a key of any size") } # [inline (always)] fn update (& mut self , data : & [u8]) { Update :: update (self , data) ; } # [inline (always)] fn finalize (self) -> Output < Self > { self . finalize_fixed () } }
};
}
