// Generated macro for macro_862 (macro)
macro_rules! Depcrate_base_indexingmacro_862 {
() => {
// Module: crate::base::indexing
// Provides: {"macro_862"}
// Dependencies: {}
impl_index_pairs ! { index R with { [<> usize => U1] , [<> ops :: Range < usize > => Dyn] , [<> ops :: RangeFrom < usize > => Dyn] , [<> ops :: RangeFull => R] , [<> ops :: RangeInclusive < usize > => Dyn] , [<> ops :: RangeTo < usize > => Dyn] , [<> ops :: RangeToInclusive < usize > => Dyn] , [< I : Dim > ops :: RangeFrom < I > => DimDiff < R , I > where R : DimSub < I >] , } index C with { [<> usize => U1] , [<> ops :: Range < usize > => Dyn] , [<> ops :: RangeFrom < usize > => Dyn] , [<> ops :: RangeFull => C] , [<> ops :: RangeInclusive < usize > => Dyn] , [<> ops :: RangeTo < usize > => Dyn] , [<> ops :: RangeToInclusive < usize > => Dyn] , [< J : DimName > ops :: RangeFrom < J > => DimDiff < C , J > where C : DimSub < J >] , } }
};
}
