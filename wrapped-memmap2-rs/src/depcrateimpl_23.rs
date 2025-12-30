// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
# [cfg (unix)] impl < T > MmapAsRawDesc for & T where T : AsRawFd , { fn as_raw_desc (& self) -> MmapRawDescriptor { MmapRawDescriptor (self . as_raw_fd ()) } }
};
}
