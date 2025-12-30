// Generated macro for impl_150 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_150 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_150"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < C : Dim > Allocator < Dyn , C > for DefaultAllocator { type Buffer < T : Scalar > = VecStorage < T , Dyn , C > ; type BufferUninit < T : Scalar > = VecStorage < MaybeUninit < T > , Dyn , C > ; # [inline] fn allocate_uninit < T : Scalar > (nrows : Dyn , ncols : C) -> VecStorage < MaybeUninit < T > , Dyn , C > { let mut data = Vec :: new () ; let length = nrows . value () * ncols . value () ; data . reserve_exact (length) ; data . resize_with (length , MaybeUninit :: uninit) ; VecStorage :: new (nrows , ncols , data) } # [inline] unsafe fn assume_init < T : Scalar > (uninit : VecStorage < MaybeUninit < T > , Dyn , C > ,) -> VecStorage < T , Dyn , C > { let (nrows , ncols) = uninit . shape () ; let vec : Vec < _ > = uninit . into () ; let mut md = ManuallyDrop :: new (vec) ; let new_data = Vec :: from_raw_parts (md . as_mut_ptr () as * mut _ , md . len () , md . capacity ()) ; VecStorage :: new (nrows , ncols , new_data) } # [inline] fn allocate_from_iterator < T : Scalar , I : IntoIterator < Item = T > > (nrows : Dyn , ncols : C , iter : I ,) -> Self :: Buffer < T > { let it = iter . into_iter () ; let res : Vec < T > = it . collect () ; assert ! (res . len () == nrows . value () * ncols . value () , "Allocation from iterator error: the iterator did not yield the correct number of elements.") ; VecStorage :: new (nrows , ncols , res) } }
};
}
