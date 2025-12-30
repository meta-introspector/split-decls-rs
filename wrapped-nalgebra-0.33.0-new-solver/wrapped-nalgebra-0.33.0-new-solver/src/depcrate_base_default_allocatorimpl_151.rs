// Generated macro for impl_151 (impl)
macro_rules! Depcrate_base_default_allocatorimpl_151 {
() => {
// Module: crate::base::default_allocator
// Provides: {"impl_151"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < R : DimName > Allocator < R , Dyn > for DefaultAllocator { type Buffer < T : Scalar > = VecStorage < T , R , Dyn > ; type BufferUninit < T : Scalar > = VecStorage < MaybeUninit < T > , R , Dyn > ; # [inline] fn allocate_uninit < T : Scalar > (nrows : R , ncols : Dyn) -> VecStorage < MaybeUninit < T > , R , Dyn > { let mut data = Vec :: new () ; let length = nrows . value () * ncols . value () ; data . reserve_exact (length) ; data . resize_with (length , MaybeUninit :: uninit) ; VecStorage :: new (nrows , ncols , data) } # [inline] unsafe fn assume_init < T : Scalar > (uninit : VecStorage < MaybeUninit < T > , R , Dyn > ,) -> VecStorage < T , R , Dyn > { let (nrows , ncols) = uninit . shape () ; let vec : Vec < _ > = uninit . into () ; let mut md = ManuallyDrop :: new (vec) ; let new_data = Vec :: from_raw_parts (md . as_mut_ptr () as * mut _ , md . len () , md . capacity ()) ; VecStorage :: new (nrows , ncols , new_data) } # [inline] fn allocate_from_iterator < T : Scalar , I : IntoIterator < Item = T > > (nrows : R , ncols : Dyn , iter : I ,) -> Self :: Buffer < T > { let it = iter . into_iter () ; let res : Vec < T > = it . collect () ; assert ! (res . len () == nrows . value () * ncols . value () , "Allocation from iterator error: the iterator did not yield the correct number of elements.") ; VecStorage :: new (nrows , ncols , res) } }
};
}
