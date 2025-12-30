// Generated macro for Mode (enum)
macro_rules! Depcrate_outputMode {
() => {
// Module: crate::output
// Provides: {"Mode"}
// Dependencies: {}
# [doc = " The **mode** is the “type” of output."] # [derive (PartialEq , Eq , Debug)] # [allow (clippy :: large_enum_variant)] pub enum Mode { Grid (grid :: Options) , Details (details :: Options) , GridDetails (grid_details :: Options) , Lines , }
};
}
