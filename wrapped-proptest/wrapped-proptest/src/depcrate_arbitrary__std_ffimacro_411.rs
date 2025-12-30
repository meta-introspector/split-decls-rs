// Generated macro for macro_411 (macro)
macro_rules! Depcrate_arbitrary__std_ffimacro_411 {
() => {
// Module: crate::arbitrary::_std::ffi
// Provides: {"macro_411"}
// Dependencies: {}
arbitrary ! (CString , SFnPtrMap < VecStrategy < RangeInclusive < u8 >>, Self >, SizeRange ; args => static_map (vec (1 ..=:: std :: u8 :: MAX , args + 1) , | mut vec | { vec . pop () . unwrap () ; Self :: new (vec) . unwrap () })) ;
};
}
