// Generated macro for impl_88 (impl)
macro_rules! Depcrate_cache_delta_traverse_utilimpl_88 {
() => {
// Module: crate::cache::delta::traverse::util
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'a , T > ItemSliceSync < 'a , T > where T : Send , { pub (super) fn new (items : & 'a mut [T]) -> Self { ItemSliceSync { items : items . as_mut_ptr () , # [cfg (debug_assertions)] len : items . len () , phantom : PhantomData , } } # [allow (unsafe_code)] pub (super) unsafe fn get_mut (& self , index : usize) -> & 'a mut T { # [cfg (debug_assertions)] if index >= self . len { panic ! ("index out of bounds: the len is {} but the index is {index}" , self . len) ; } unsafe { & mut * self . items . add (index) } } }
};
}
