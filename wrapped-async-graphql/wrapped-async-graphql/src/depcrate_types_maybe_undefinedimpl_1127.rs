// Generated macro for impl_1127 (impl)
macro_rules! Depcrate_types_maybe_undefinedimpl_1127 {
() => {
// Module: crate::types::maybe_undefined
// Provides: {"impl_1127"}
// Dependencies: {}
impl < T , E > MaybeUndefined < Result < T , E > > { # [doc = " Transposes a `MaybeUndefined` of a [`Result`] into a [`Result`] of a"] # [doc = " `MaybeUndefined`."] # [doc = ""] # [doc = " [`MaybeUndefined::Undefined`] will be mapped to"] # [doc = " [`Ok`]`(`[`MaybeUndefined::Undefined`]`)`. [`MaybeUndefined::Null`]"] # [doc = " will be mapped to [`Ok`]`(`[`MaybeUndefined::Null`]`)`."] # [doc = " [`MaybeUndefined::Value`]`(`[`Ok`]`(_))` and"] # [doc = " [`MaybeUndefined::Value`]`(`[`Err`]`(_))` will be mapped to"] # [doc = " [`Ok`]`(`[`MaybeUndefined::Value`]`(_))` and [`Err`]`(_)`."] # [inline] pub fn transpose (self) -> Result < MaybeUndefined < T > , E > { match self { MaybeUndefined :: Undefined => Ok (MaybeUndefined :: Undefined) , MaybeUndefined :: Null => Ok (MaybeUndefined :: Null) , MaybeUndefined :: Value (Ok (v)) => Ok (MaybeUndefined :: Value (v)) , MaybeUndefined :: Value (Err (e)) => Err (e) , } } }
};
}
