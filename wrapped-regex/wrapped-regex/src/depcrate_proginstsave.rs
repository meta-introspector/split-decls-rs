// Generated macro for InstSave (struct)
macro_rules! Depcrate_progInstSave {
() => {
// Module: crate::prog
// Provides: {"InstSave"}
// Dependencies: {}
# [doc = " Representation of the Save instruction."] # [derive (Clone , Debug)] pub struct InstSave { # [doc = " The next location to execute in the program."] pub goto : InstPtr , # [doc = " The capture slot (there are two slots for every capture in a regex,"] # [doc = " including the zeroth capture for the entire match)."] pub slot : usize , }
};
}
