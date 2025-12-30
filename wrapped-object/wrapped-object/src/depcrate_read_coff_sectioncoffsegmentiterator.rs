// Generated macro for CoffSegmentIterator (struct)
macro_rules! Depcrate_read_coff_sectionCoffSegmentIterator {
() => {
// Module: crate::read::coff::section
// Provides: {"CoffSegmentIterator"}
// Dependencies: {}
# [doc = " An iterator for the loadable sections in a [`CoffFile`]."] # [derive (Debug)] pub struct CoffSegmentIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { pub (super) file : & 'file CoffFile < 'data , R , Coff > , pub (super) iter : slice :: Iter < 'data , pe :: ImageSectionHeader > , }
};
}
