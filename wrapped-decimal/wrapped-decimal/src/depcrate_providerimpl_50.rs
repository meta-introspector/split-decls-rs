// Generated macro for impl_50 (impl)
macro_rules! Depcrate_providerimpl_50 {
() => {
// Module: crate::provider
// Provides: {"impl_50"}
// Dependencies: {}
impl DecimalSymbols < 'static > { # [doc = " Create a new en-US format for use in testing"] # [cfg (feature = "datagen")] pub fn new_en_for_testing () -> Self { let strings = DecimalSymbolStrsBuilder { minus_sign_prefix : VarZeroCow :: new_borrowed ("-") , minus_sign_suffix : VarZeroCow :: new_borrowed ("") , plus_sign_prefix : VarZeroCow :: new_borrowed ("+") , plus_sign_suffix : VarZeroCow :: new_borrowed ("") , decimal_separator : VarZeroCow :: new_borrowed (".") , grouping_separator : VarZeroCow :: new_borrowed (",") , numsys : VarZeroCow :: new_borrowed ("latn") , } ; Self { strings : VarZeroCow :: from_encodeable (& strings) , grouping_sizes : GroupingSizes { primary : 3 , secondary : 3 , min_grouping : 1 , } , } } }
};
}
