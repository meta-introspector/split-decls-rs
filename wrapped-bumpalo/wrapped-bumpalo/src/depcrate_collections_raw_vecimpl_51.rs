// Generated macro for impl_51 (impl)
macro_rules! Depcrate_collections_raw_vecimpl_51 {
() => {
// Module: crate::collections::raw_vec
// Provides: {"impl_51"}
// Dependencies: {}
# [cfg (feature = "boxed")] impl < 'a , T > RawVec < 'a , T > { # [doc = " Converts the entire buffer into `Box<[T]>`."] # [doc = ""] # [doc = " Note that this will correctly reconstitute any `cap` changes"] # [doc = " that may have been performed. (See description of type for details.)"] # [doc = ""] # [doc = " # Undefined Behavior"] # [doc = ""] # [doc = " All elements of `RawVec<T>` must be initialized. Notice that"] # [doc = " the rules around uninitialized boxed values are not finalized yet,"] # [doc = " but until they are, it is advisable to avoid them."] pub unsafe fn into_box (self) -> crate :: boxed :: Box < 'a , [T] > { use crate :: boxed :: Box ; let slice = core :: slice :: from_raw_parts_mut (self . ptr () , self . cap) ; let output : Box < 'a , [T] > = Box :: from_raw (slice) ; mem :: forget (self) ; output } }
};
}
