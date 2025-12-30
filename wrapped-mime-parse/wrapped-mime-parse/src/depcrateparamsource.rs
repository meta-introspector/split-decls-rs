// Generated macro for ParamSource (enum)
macro_rules! DepcrateParamSource {
() => {
// Module: crate
// Provides: {"ParamSource"}
// Dependencies: {}
# [derive (Clone)] pub enum ParamSource { None , Utf8 (u16) , One (u16 , IndexedPair) , Two (u16 , IndexedPair , IndexedPair) , Custom (u16 , Vec < IndexedPair >) , }
};
}
