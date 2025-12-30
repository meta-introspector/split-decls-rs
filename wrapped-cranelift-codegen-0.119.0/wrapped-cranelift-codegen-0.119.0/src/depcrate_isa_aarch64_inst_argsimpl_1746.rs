// Generated macro for impl_1746 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1746 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1746"}
// Dependencies: {}
impl PrettyPrint for PairAMode { fn pretty_print (& self , _ : u8) -> String { match self { & PairAMode :: SignedOffset { reg , simm7 } => { let reg = pretty_print_reg (reg) ; if simm7 . value != 0 { let simm7 = simm7 . pretty_print (8) ; format ! ("[{reg}, {simm7}]") } else { format ! ("[{reg}]") } } & PairAMode :: SPPreIndexed { simm7 } => { let simm7 = simm7 . pretty_print (8) ; format ! ("[sp, {simm7}]!") } & PairAMode :: SPPostIndexed { simm7 } => { let simm7 = simm7 . pretty_print (8) ; format ! ("[sp], {simm7}") } } } }
};
}
