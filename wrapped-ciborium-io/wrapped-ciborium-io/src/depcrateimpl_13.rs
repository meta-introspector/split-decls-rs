// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T : std :: io :: Write > Write for T { type Error = std :: io :: Error ; # [inline] fn write_all (& mut self , data : & [u8]) -> Result < () , Self :: Error > { self . write_all (data) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { self . flush () } }
};
}
