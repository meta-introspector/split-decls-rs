// Generated macro for impl_83 (impl)
macro_rules! Depcrate_drop_listimpl_83 {
() => {
// Module: crate::drop_list
// Provides: {"impl_83"}
// Dependencies: {}
impl < T > DropItem < [T ; 0] > { pub unsafe fn init_slice < 'a > (mut ptr : NonNull < DropItem < [T ; 0] > > , count : usize ,) -> (& 'a mut Self , & 'a mut [T]) { debug_assert_ne ! (count , 0 , "DropItem<[T]> should not be constructed with count 0") ; ptr :: write (ptr . as_ptr () . cast () , Drops { count , drop : drop_from_item :: < T > , next : None , } ,) ; let slice = core :: slice :: from_raw_parts_mut (ptr . as_ptr () . add (1) . cast () , count) ; (ptr . as_mut () , slice) } }
};
}
