// Generated macro for define_registers (macro)
macro_rules! Depcrate_isa_pulley_shared_inst_regsdefine_registers {
() => {
// Module: crate::isa::pulley_shared::inst::regs
// Provides: {"define_registers"}
// Dependencies: {}
macro_rules ! define_registers { ($ ($ reg : expr => $ readable : ident , $ writable : ident ;) *) => { $ (# [inline] # [allow (dead_code)] pub fn $ readable () -> Reg { $ reg } # [inline] # [allow (dead_code)] pub fn $ writable () -> Writable < Reg > { Writable :: from_reg ($ readable ()) }) * } ; }
};
}
