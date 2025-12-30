// Generated macro for MapOkOrElseFn (type)
macro_rules! Depcrate_fnsMapOkOrElseFn {
() => {
// Module: crate::fns
// Provides: {"MapOkOrElseFn"}
// Dependencies: {}
pub (crate) type MapOkOrElseFn < F , G > = ChainFn < MapOkFn < F > , ChainFn < MapErrFn < G > , MergeResultFn > > ;
};
}
