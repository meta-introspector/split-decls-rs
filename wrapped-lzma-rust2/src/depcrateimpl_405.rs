// Generated macro for impl_405 (impl)
macro_rules! Depcrateimpl_405 {
() => {
// Module: crate
// Provides: {"impl_405"}
// Dependencies: {}
impl < T : Write > ByteWriter for T { # [inline (always)] fn write_u8 (& mut self , value : u8) -> Result < () > { self . write_all (& [value]) } # [inline (always)] fn write_u16 (& mut self , value : u16) -> Result < () > { self . write_all (& value . to_le_bytes ()) } # [inline (always)] fn write_u32 (& mut self , value : u32) -> Result < () > { self . write_all (& value . to_le_bytes ()) } # [inline (always)] fn write_u64 (& mut self , value : u64) -> Result < () > { self . write_all (& value . to_le_bytes ()) } }
};
}
