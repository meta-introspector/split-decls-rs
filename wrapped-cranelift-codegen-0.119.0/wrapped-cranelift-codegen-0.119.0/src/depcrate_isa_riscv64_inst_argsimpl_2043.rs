// Generated macro for impl_2043 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2043 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2043"}
// Dependencies: {}
impl Display for AMode { fn fmt (& self , f : & mut Formatter < '_ >) -> Result { match self { & AMode :: RegOffset (r , offset , ..) => { write ! (f , "{}({})" , offset , reg_name (r)) } & AMode :: SPOffset (offset , ..) => { write ! (f , "{offset}(sp)") } & AMode :: SlotOffset (offset , ..) => { write ! (f , "{offset}(slot)") } & AMode :: IncomingArg (offset) => { write ! (f , "-{offset}(incoming_arg)") } & AMode :: FPOffset (offset , ..) => { write ! (f , "{offset}(fp)") } & AMode :: Const (addr , ..) => { write ! (f , "[const({})]" , addr . as_u32 ()) } & AMode :: Label (label) => { write ! (f , "[label{}]" , label . as_u32 ()) } } } }
};
}
