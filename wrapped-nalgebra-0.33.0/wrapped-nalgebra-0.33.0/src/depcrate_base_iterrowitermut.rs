// Generated macro for RowIterMut (struct)
macro_rules! Depcrate_base_iterRowIterMut {
() => {
// Module: crate::base::iter
// Provides: {"RowIterMut"}
// Dependencies: {}
# [doc = " An iterator through the mutable rows of a matrix."] # [derive (Debug)] pub struct RowIterMut < 'a , T , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > { mat : * mut Matrix < T , R , C , S > , curr : usize , phantom : PhantomData < & 'a mut Matrix < T , R , C , S > > , }
};
}
