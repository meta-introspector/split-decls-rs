// Generated macro for SegmentIter (struct)
macro_rules! Depcrate_macosSegmentIter {
() => {
// Module: crate::macos
// Provides: {"SegmentIter"}
// Dependencies: {}
# [doc = " An iterator over Mach-O segments."] # [derive (Debug)] pub struct SegmentIter < 'a > { phantom : PhantomData < & 'a SharedLibrary < 'a > > , commands : * const libc :: load_command , num_commands : usize , }
};
}
