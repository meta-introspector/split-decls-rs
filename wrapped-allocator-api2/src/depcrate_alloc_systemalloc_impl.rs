// Generated macro for alloc_impl (function)
macro_rules! Depcrate_alloc_systemalloc_impl {
() => {
// Module: crate::alloc::system
// Provides: {"alloc_impl"}
// Dependencies: {}
# [inline (always)] fn alloc_impl (layout : Layout , zeroed : bool) -> Result < NonNull < [u8] > , AllocError > { match layout . size () { 0 => Ok (unsafe { NonNull :: new_unchecked (core :: ptr :: slice_from_raw_parts_mut (invalid_mut (layout . align ()) , 0 ,)) }) , size => unsafe { let raw_ptr = if zeroed { System . alloc_zeroed (layout) } else { System . alloc (layout) } ; let ptr = NonNull :: new (raw_ptr) . ok_or (AllocError) ? ; Ok (NonNull :: new_unchecked (core :: ptr :: slice_from_raw_parts_mut (ptr . as_ptr () , size ,))) } , } }
};
}
