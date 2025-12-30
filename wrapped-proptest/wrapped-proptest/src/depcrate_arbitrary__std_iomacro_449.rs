// Generated macro for macro_449 (macro)
macro_rules! Depcrate_arbitrary__std_iomacro_449 {
() => {
// Module: crate::arbitrary::_std::io
// Provides: {"macro_449"}
// Dependencies: {}
arbitrary ! (SeekFrom , TupleUnion < (WA < SMapped < u64 , SeekFrom >>, WA < SMapped < i64 , SeekFrom >>, WA < SMapped < i64 , SeekFrom >>,) >; prop_oneof ! [static_map (any ::< u64 > () , SeekFrom :: Start) , static_map (any ::< i64 > () , SeekFrom :: End) , static_map (any ::< i64 > () , SeekFrom :: Current)]) ;
};
}
