// Generated macro for impl_225 (impl)
macro_rules! Depcrate_base_iterimpl_225 {
() => {
// Module: crate::base::iter
// Provides: {"impl_225"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S : 'a + RawStorageMut < T , R , C > > ColumnIterMut < 'a , T , R , C , S > { pub (crate) fn new (mat : & 'a mut Matrix < T , R , C , S >) -> Self { let range = 0 .. mat . ncols () ; ColumnIterMut { mat , range , phantom : Default :: default () , } } # [cfg (feature = "rayon")] pub (crate) fn split_at (self , index : usize) -> (Self , Self) { let split_pos = (self . range . start + index) . min (self . range . end) ; let left_iter = ColumnIterMut { mat : self . mat , range : self . range . start .. split_pos , phantom : Default :: default () , } ; let right_iter = ColumnIterMut { mat : self . mat , range : split_pos .. self . range . end , phantom : Default :: default () , } ; (left_iter , right_iter) } fn ncols (& self) -> usize { unsafe { (* self . mat) . ncols () } } }
};
}
