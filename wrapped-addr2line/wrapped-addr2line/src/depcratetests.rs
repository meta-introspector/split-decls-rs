// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn context_is_send () { fn assert_is_send < T : Send > () { } assert_is_send :: < crate :: Context < gimli :: read :: EndianSlice < '_ , gimli :: LittleEndian > > > () ; } }
};
}
