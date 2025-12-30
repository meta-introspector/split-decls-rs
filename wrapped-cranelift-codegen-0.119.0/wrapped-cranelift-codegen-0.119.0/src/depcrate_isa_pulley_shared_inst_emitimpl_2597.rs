// Generated macro for impl_2597 (impl)
macro_rules! Depcrate_isa_pulley_shared_inst_emitimpl_2597 {
() => {
// Module: crate::isa::pulley_shared::inst::emit
// Provides: {"impl_2597"}
// Dependencies: {}
impl EmitInfo { pub (crate) fn new (call_conv : isa :: CallConv , shared_flags : settings :: Flags , isa_flags : crate :: isa :: pulley_shared :: settings :: Flags ,) -> Self { Self { call_conv , shared_flags , isa_flags , } } fn endianness (& self , flags : MemFlags) -> Endianness { flags . endianness (self . isa_flags . endianness ()) } }
};
}
