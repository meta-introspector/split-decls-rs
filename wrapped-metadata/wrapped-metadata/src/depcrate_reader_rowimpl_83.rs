// Generated macro for impl_83 (impl)
macro_rules! Depcrate_reader_rowimpl_83 {
() => {
// Module: crate::reader::row
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a , R : AsRow < 'a > > RowIterator < 'a , R > { pub (crate) fn new (index : & 'a TypeIndex , file : usize , rows : std :: ops :: Range < usize >) -> Self { Self { index , file , rows , phantom : std :: marker :: PhantomData , } } }
};
}
