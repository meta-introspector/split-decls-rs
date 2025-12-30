// Generated macro for impl_149 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_149 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_149"}
// Dependencies: {}
impl < const R : usize , const C : usize > Allocator < Const < R > , Const < C > > for DefaultAllocator { type Buffer < T : Scalar > = ArrayStorage < T , R , C > ; type BufferUninit < T : Scalar > = ArrayStorage < MaybeUninit < T > , R , C > ; # [inline (always)] fn allocate_uninit < T : Scalar > (_ : Const < R > , _ : Const < C >) -> ArrayStorage < MaybeUninit < T > , R , C > { let array : [[MaybeUninit < T > ; R] ; C] = unsafe { MaybeUninit :: uninit () . assume_init () } ; ArrayStorage (array) } # [inline (always)] unsafe fn assume_init < T : Scalar > (uninit : ArrayStorage < MaybeUninit < T > , R , C > ,) -> ArrayStorage < T , R , C > { ArrayStorage ((& uninit as * const _ as * const [_ ; C]) . read ()) } # [inline] fn allocate_from_iterator < T : Scalar , I : IntoIterator < Item = T > > (nrows : Const < R > , ncols : Const < C > , iter : I ,) -> Self :: Buffer < T > { let mut res = Self :: allocate_uninit (nrows , ncols) ; let mut count = 0 ; let res_slice = unsafe { res . as_mut_slice_unchecked () } ; for (res , e) in res_slice . iter_mut () . zip (iter . into_iter ()) { * res = MaybeUninit :: new (e) ; count += 1 ; } assert ! (count == nrows . value () * ncols . value () , "Matrix init. from iterator: iterator not long enough.") ; unsafe { < Self as Allocator < Const < R > , Const < C > > > :: assume_init (res) } } }
};
}
