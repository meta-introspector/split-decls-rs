// Generated macro for CoffRelocationIterator (struct)
macro_rules! Depcrate_read_coff_relocationCoffRelocationIterator {
() => {
// Module: crate::read::coff::relocation
// Provides: {"CoffRelocationIterator"}
// Dependencies: {}
# [doc = " An iterator for the relocations in a [`CoffSection`](super::CoffSection)."] pub struct CoffRelocationIterator < 'data , 'file , R : ReadRef < 'data > = & 'data [u8] , Coff : CoffHeader = pe :: ImageFileHeader , > { pub (super) file : & 'file CoffFile < 'data , R , Coff > , pub (super) iter : slice :: Iter < 'data , pe :: ImageRelocation > , }
};
}
