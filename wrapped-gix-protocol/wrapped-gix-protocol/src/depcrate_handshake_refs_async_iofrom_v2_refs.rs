// Generated macro for from_v2_refs (function)
macro_rules! Depcrate_handshake_refs_async_iofrom_v2_refs {
() => {
// Module: crate::handshake::refs::async_io
// Provides: {"from_v2_refs"}
// Dependencies: {}
# [doc = " Parse refs from the given input line by line. Protocol V2 is required for this to succeed."] pub async fn from_v2_refs (in_refs : & mut dyn ReadlineBufRead) -> Result < Vec < Ref > , Error > { let mut out_refs = Vec :: new () ; while let Some (line) = in_refs . readline () . await . transpose () ? . transpose () ? . and_then (| l | l . as_bstr ()) { out_refs . push (refs :: shared :: parse_v2 (line) ?) ; } Ok (out_refs) }
};
}
