// Generated macro for impl_33 (impl)
macro_rules! Depcrate_counterimpl_33 {
() => {
// Module: crate::counter
// Provides: {"impl_33"}
// Dependencies: {}
impl < W : Write > Write for CounterWriter < W > { # [inline (always)] fn write (& mut self , buf : & [u8]) -> Result < usize , io :: Error > { let n = self . wtr . write (buf) ? ; self . count += n as u64 ; Ok (n) } # [inline] fn flush (& mut self) -> Result < () , io :: Error > { self . wtr . flush () } }
};
}
