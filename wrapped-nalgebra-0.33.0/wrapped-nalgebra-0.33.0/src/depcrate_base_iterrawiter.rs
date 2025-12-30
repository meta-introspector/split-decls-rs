// Generated macro for RawIter (struct)
macro_rules! Depcrate_base_iterRawIter {
() => {
// Module: crate::base::iter
// Provides: {"RawIter"}
// Dependencies: {}
# [derive (Clone , Debug)] struct RawIter < Ptr , T , R : Dim , C : Dim , RStride : Dim , CStride : Dim > { ptr : Ptr , inner_ptr : Ptr , inner_end : Ptr , size : usize , strides : (RStride , CStride) , _phantoms : PhantomData < (fn () -> T , R , C) > , }
};
}
