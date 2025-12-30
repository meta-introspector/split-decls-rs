// Generated macro for other_106 (other)
macro_rules! Depcrateother_106 {
() => {
// Module: crate
// Provides: {"other_106"}
// Dependencies: {}
# [doc = " Either a stack array with `length <= N` or a heap array"] # [doc = " whose pointer and capacity are stored here."] # [doc = ""] # [doc = " We store a `NonNull<T>` instead of a `*mut T`, so that"] # [doc = " niche-optimization can be performed and the type is covariant"] # [doc = " with respect to `T`."] # [repr (C)] pub union RawSmallVec < T , const N : usize > { inline : ManuallyDrop < MaybeUninit < [T ; N] > > , heap : (NonNull < T > , usize) , }
};
}
