// Generated macro for impl_162 (impl)
macro_rules! Depcrate_x25519impl_162 {
() => {
// Module: crate::x25519
// Provides: {"impl_162"}
// Dependencies: {}
impl Deref for DHOutput { type Target = [u8 ; DHOutput :: BYTES] ; # [doc = " Returns the output of the scalar multiplication as bytes."] # [doc = " The output is not uniform, and should be hashed before use."] fn deref (& self) -> & Self :: Target { & self . 0 } }
};
}
