// Generated macro for RowIterator (struct)
macro_rules! Depcrate_reader_rowRowIterator {
() => {
// Module: crate::reader::row
// Provides: {"RowIterator"}
// Dependencies: {}
pub struct RowIterator < 'a , R : AsRow < 'a > > { index : & 'a TypeIndex , file : usize , rows : std :: ops :: Range < usize > , phantom : std :: marker :: PhantomData < R > , }
};
}
