// Generated macro for impl_265 (impl)
macro_rules! Depcrate_machinst_bufferimpl_265 {
() => {
// Module: crate::machinst::buffer
// Provides: {"impl_265"}
// Dependencies: {}
impl MachLabel { # [doc = " Get a label for a block. (The first N MachLabels are always reserved for"] # [doc = " the N blocks in the vcode.)"] pub fn from_block (bindex : BlockIndex) -> MachLabel { MachLabel (bindex . index () as u32) } # [doc = " Creates a string representing this label, for convenience."] pub fn to_string (& self) -> String { format ! ("label{}" , self . 0) } }
};
}
