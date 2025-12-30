// Generated macro for PDA_MARKER (const)
macro_rules! DepcratePDA_MARKER {
() => {
// Module: crate
// Provides: {"PDA_MARKER"}
// Dependencies: {}
# [doc = " Marker used to find program derived addresses (PDAs)."] # [cfg (not (target_arch = "bpf"))] pub const PDA_MARKER : & [u8 ; 21] = b"ProgramDerivedAddress" ;
};
}
