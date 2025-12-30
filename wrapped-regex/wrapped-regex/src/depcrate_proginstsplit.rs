// Generated macro for InstSplit (struct)
macro_rules! Depcrate_progInstSplit {
() => {
// Module: crate::prog
// Provides: {"InstSplit"}
// Dependencies: {}
# [doc = " Representation of the Split instruction."] # [derive (Clone , Debug)] pub struct InstSplit { # [doc = " The first instruction to try. A match resulting from following goto1"] # [doc = " has precedence over a match resulting from following goto2."] pub goto1 : InstPtr , # [doc = " The second instruction to try. A match resulting from following goto1"] # [doc = " has precedence over a match resulting from following goto2."] pub goto2 : InstPtr , }
};
}
