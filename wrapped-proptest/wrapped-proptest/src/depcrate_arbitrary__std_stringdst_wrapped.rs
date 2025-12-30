// Generated macro for dst_wrapped (macro)
macro_rules! Depcrate_arbitrary__std_stringdst_wrapped {
() => {
// Module: crate::arbitrary::_std::string
// Provides: {"dst_wrapped"}
// Dependencies: {}
macro_rules ! dst_wrapped { ($ ($ w : ident) ,*) => { $ (arbitrary ! ($ w < str >, MapInto < StrategyFor < String >, Self >, StringParam ; a => any_with ::< String > (a) . prop_map_into ()) ;) * } ; }
};
}
