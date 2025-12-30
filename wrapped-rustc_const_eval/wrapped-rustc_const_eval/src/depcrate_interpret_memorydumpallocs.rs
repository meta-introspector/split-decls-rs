// Generated macro for DumpAllocs (struct)
macro_rules! Depcrate_interpret_memoryDumpAllocs {
() => {
// Module: crate::interpret::memory
// Provides: {"DumpAllocs"}
// Dependencies: {}
# [doc (hidden)] # [doc = " There's no way to use this directly, it's just a helper struct for the `dump_alloc(s)` methods."] pub struct DumpAllocs < 'a , 'tcx , M : Machine < 'tcx > > { ecx : & 'a InterpCx < 'tcx , M > , allocs : Vec < AllocId > , }
};
}
