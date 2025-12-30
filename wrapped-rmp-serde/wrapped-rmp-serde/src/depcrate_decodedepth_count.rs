// Generated macro for depth_count (macro)
macro_rules! Depcrate_decodedepth_count {
() => {
// Module: crate::decode
// Provides: {"depth_count"}
// Dependencies: {}
macro_rules ! depth_count (($ counter : expr , $ expr : expr) => { { $ counter -= 1 ; if $ counter == 0 { return Err (Error :: DepthLimitExceeded) } let res = $ expr ; $ counter += 1 ; res } }) ;
};
}
