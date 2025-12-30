// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (feature = "boringssl-boring-crate")] impl < F : BufFactory > AsMut < boring :: ssl :: SslRef > for Connection < F > { fn as_mut (& mut self) -> & mut boring :: ssl :: SslRef { self . handshake . ssl_mut () } }
};
}
