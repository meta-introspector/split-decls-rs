// Generated macro for impl_82 (impl)
macro_rules! Depcrate_drop_listimpl_82 {
() => {
// Module: crate::drop_list
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > DropItem < T > { pub unsafe fn init_value < 'a , I > (mut ptr : NonNull < DropItem < T > > , init : I , f : impl FnOnce (& mut MaybeUninit < T > , I) ,) -> & 'a mut Self { let drops_ptr = addr_of_mut ! ((* ptr . as_ptr ()) . drops) ; f (& mut * addr_of_mut ! ((* ptr . as_ptr ()) . value) . cast () , init) ; ptr :: write (drops_ptr , Drops { count : 1 , drop : drop_from_item :: < T > , next : None , } ,) ; ptr . as_mut () } }
};
}
