// Generated macro for dst_wrapped (macro)
macro_rules! Depcrate_arbitrary__std_ffidst_wrapped {
() => {
// Module: crate::arbitrary::_std::ffi
// Provides: {"dst_wrapped"}
// Dependencies: {}
macro_rules ! dst_wrapped { ($ ($ w : ident) ,*) => { $ (arbitrary ! ($ w < CStr >, MapInto < StrategyFor < CString >, Self >, SizeRange ; a => any_with ::< CString > (a) . prop_map_into ()) ;) * $ (arbitrary ! ($ w < OsStr >, MapInto < StrategyFor < OsString >, Self >, < String as Arbitrary >:: Parameters ; a => any_with ::< OsString > (a) . prop_map_into ()) ;) * } ; }
};
}
