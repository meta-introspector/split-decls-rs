// Generated macro for impl_65 (impl)
macro_rules! Depcrate_linuximpl_65 {
() => {
// Module: crate::linux
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'a > SharedLibraryTrait for SharedLibrary < 'a > { type Segment = Segment < 'a > ; type SegmentIter = SegmentIter < 'a > ; # [inline] fn name (& self) -> & OsStr { OsStr :: from_bytes (self . name . to_bytes ()) } fn id (& self) -> Option < SharedLibraryId > { for segment in self . note_segments () { for (note_type , note_name , note_descriptor) in unsafe { segment . notes (self) } { if note_type == NT_GNU_BUILD_ID && note_name == b"GNU\0" { return Some (SharedLibraryId :: GnuBuildId (note_descriptor . to_vec ())) ; } } } None } # [inline] fn segments (& self) -> Self :: SegmentIter { SegmentIter { inner : self . headers . iter () , } } # [inline] fn virtual_memory_bias (& self) -> Bias { Bias (self . addr as usize) } # [inline] fn each < F , C > (f : F) where F : FnMut (& Self) -> C , C : Into < IterationControl > , { let mut state = IterState { f : f , panic : None , idx : 0 , } ; unsafe { libc :: dl_iterate_phdr (Some (Self :: callback :: < F , C >) , & mut state as * mut _ as * mut _) ; } if let Some (panic) = state . panic { panic :: resume_unwind (panic) ; } } }
};
}
