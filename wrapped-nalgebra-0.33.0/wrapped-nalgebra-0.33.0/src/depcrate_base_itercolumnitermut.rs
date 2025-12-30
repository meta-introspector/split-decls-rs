// Generated macro for ColumnIterMut (struct)
macro_rules! Depcrate_base_iterColumnIterMut {
() => {
// Module: crate::base::iter
// Provides: {"ColumnIterMut"}
// Dependencies: {}
# [doc = " An iterator through the mutable columns of a matrix."] # [derive (Debug)] pub struct ColumnIterMut < 'a , T , R : Dim , C : Dim , S : RawStorageMut < T , R , C > > { mat : * mut Matrix < T , R , C , S > , range : Range < usize > , phantom : PhantomData < & 'a mut Matrix < T , R , C , S > > , }
};
}
