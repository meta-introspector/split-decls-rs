// Generated macro for impl_85 (impl)
macro_rules! Depcrate_machinst_lowerimpl_85 {
() => {
// Module: crate::machinst::lower
// Provides: {"impl_85"}
// Dependencies: {}
impl InputSourceInst { # [doc = " Get the instruction and output index for this source, whether"] # [doc = " we are its single or one of many users."] pub fn as_inst (& self) -> Option < (Inst , usize) > { match self { & InputSourceInst :: UniqueUse (inst , output_idx) | & InputSourceInst :: Use (inst , output_idx) => Some ((inst , output_idx)) , & InputSourceInst :: None => None , } } }
};
}
