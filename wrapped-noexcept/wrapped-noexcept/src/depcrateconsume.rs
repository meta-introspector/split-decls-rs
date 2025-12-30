// Generated macro for consume (function)
macro_rules! Depcrateconsume {
() => {
// Module: crate
// Provides: {"consume"}
// Dependencies: {}
pub fn consume (test : & ITest) -> Result < () > { unsafe extern "system" { fn consume (test : Ref < ITest >) -> HRESULT ; } unsafe { consume (test . into ()) . ok () } }
};
}
