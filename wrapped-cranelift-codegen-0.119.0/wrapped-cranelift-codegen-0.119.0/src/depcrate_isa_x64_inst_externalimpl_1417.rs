// Generated macro for impl_1417 (impl)
macro_rules! Depcrate_isa_x64_inst_externalimpl_1417 {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"impl_1417"}
// Dependencies: {}
impl asm :: CodeSink for MachBuffer < Inst > { fn put1 (& mut self , value : u8) { self . put1 (value) } fn put2 (& mut self , value : u16) { self . put2 (value) } fn put4 (& mut self , value : u32) { self . put4 (value) } fn put8 (& mut self , value : u64) { self . put8 (value) } fn current_offset (& self) -> u32 { self . cur_offset () } fn use_label_at_offset (& mut self , offset : u32 , label : asm :: Label) { self . use_label_at_offset (offset , label . into () , LabelUse :: JmpRel32) ; } fn add_trap (& mut self , code : asm :: TrapCode) { self . add_trap (code . into ()) ; } fn get_label_for_constant (& mut self , c : asm :: Constant) -> asm :: Label { self . get_label_for_constant (c . into ()) . into () } }
};
}
