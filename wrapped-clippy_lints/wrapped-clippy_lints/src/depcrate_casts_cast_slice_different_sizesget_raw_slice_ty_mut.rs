// Generated macro for get_raw_slice_ty_mut (function)
macro_rules! Depcrate_casts_cast_slice_different_sizesget_raw_slice_ty_mut {
() => {
// Module: crate::casts::cast_slice_different_sizes
// Provides: {"get_raw_slice_ty_mut"}
// Dependencies: {}
# [doc = " Returns the type T of the pointed to *const [T] or *mut [T] and the mutability of the slice if"] # [doc = " the type is one of those slices"] fn get_raw_slice_ty_mut (ty : Ty < '_ >) -> Option < TypeAndMut < '_ > > { match ty . kind () { ty :: RawPtr (slice_ty , mutbl) => match slice_ty . kind () { ty :: Slice (ty) => Some (TypeAndMut { ty : * ty , mutbl : * mutbl }) , _ => None , } , _ => None , } }
};
}
