// Generated macro for OffsetMode (enum)
macro_rules! Depcrate_interpret_projectionOffsetMode {
() => {
// Module: crate::interpret::projection
// Provides: {"OffsetMode"}
// Dependencies: {}
# [doc = " Describes the constraints placed on offset-projections."] # [derive (Copy , Clone , Debug)] pub enum OffsetMode { # [doc = " The offset has to be inbounds, like `ptr::offset`."] Inbounds , # [doc = " No constraints, just wrap around the edge of the address space."] Wrapping , }
};
}
