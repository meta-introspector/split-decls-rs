// Generated macro for produce (function)
macro_rules! Depcrateproduce {
() => {
// Module: crate
// Provides: {"produce"}
// Dependencies: {}
pub fn produce () -> Result < ITest > { unsafe extern "system" { fn produce (test : OutRef < ITest >) -> HRESULT ; } unsafe { let mut test = None ; produce ((& mut test) . into ()) . ok () ? ; Type :: from_default (& test) } }
};
}
