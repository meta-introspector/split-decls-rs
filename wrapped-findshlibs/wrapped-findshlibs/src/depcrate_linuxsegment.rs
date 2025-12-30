// Generated macro for Segment (struct)
macro_rules! Depcrate_linuxSegment {
() => {
// Module: crate::linux
// Provides: {"Segment"}
// Dependencies: {}
# [doc = " A mapped segment in an ELF file."] # [derive (Debug)] pub struct Segment < 'a > { phdr : * const Phdr , shlib : PhantomData < & 'a SharedLibrary < 'a > > , }
};
}
