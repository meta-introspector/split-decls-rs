// Generated macro for inputs (function)
macro_rules! Depcrate_testinputs {
() => {
// Module: crate::test
// Provides: {"inputs"}
// Dependencies: {}
fn inputs () -> impl Strategy < Value = Vec < (u32 , u32) > > { prop :: collection :: vec ((0_u32 .. 100 , 0_u32 .. 100) , 1 .. 500) }
};
}
