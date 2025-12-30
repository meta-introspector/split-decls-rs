// Generated macro for emit_reloc (function)
macro_rules! Depcrate_isa_x64_inst_emitemit_reloc {
() => {
// Module: crate::isa::x64::inst::emit
// Provides: {"emit_reloc"}
// Dependencies: {}
# [doc = " Emits a relocation, attaching the current source location as well."] fn emit_reloc (sink : & mut MachBuffer < Inst > , kind : Reloc , name : & ExternalName , addend : Addend) { sink . add_reloc (kind , name , addend) ; }
};
}
