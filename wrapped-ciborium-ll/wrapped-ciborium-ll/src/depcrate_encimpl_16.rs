// Generated macro for impl_16 (impl)
macro_rules! Depcrate_encimpl_16 {
() => {
// Module: crate::enc
// Provides: {"impl_16"}
// Dependencies: {}
impl < W : Write > Write for Encoder < W > { type Error = W :: Error ; # [inline] fn write_all (& mut self , data : & [u8]) -> Result < () , Self :: Error > { self . 0 . write_all (data) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { self . 0 . flush () } }
};
}
