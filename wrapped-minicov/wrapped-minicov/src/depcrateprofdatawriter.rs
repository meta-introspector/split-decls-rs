// Generated macro for ProfDataWriter (struct)
macro_rules! DepcrateProfDataWriter {
() => {
// Module: crate
// Provides: {"ProfDataWriter"}
// Dependencies: {}
# [allow (non_snake_case)] # [repr (C)] struct ProfDataWriter { Write : unsafe extern "C" fn (This : * mut ProfDataWriter , * mut ProfDataIOVec , NumIOVecs : u32) -> u32 , WriterCtx : * mut u8 , }
};
}
