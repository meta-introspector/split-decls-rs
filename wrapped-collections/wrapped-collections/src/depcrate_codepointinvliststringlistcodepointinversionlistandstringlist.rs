// Generated macro for CodePointInversionListAndStringList (struct)
macro_rules! Depcrate_codepointinvliststringlistCodePointInversionListAndStringList {
() => {
// Module: crate::codepointinvliststringlist
// Provides: {"CodePointInversionListAndStringList"}
// Dependencies: {}
# [doc = " A data structure providing a concrete implementation of a set of code points and strings,"] # [doc = " using an inversion list for the code points."] # [doc = ""] # [doc = " This is what ICU4C calls a `UnicodeSet`."] # [zerovec :: make_varule (CodePointInversionListAndStringListULE)] # [zerovec :: skip_derive (Ord)] # [zerovec :: derive (Debug)] # [derive (Debug , Eq , PartialEq , Clone , Yokeable , ZeroFrom)] # [cfg_attr (not (feature = "alloc") , zerovec :: skip_derive (ZeroMapKV , ToOwned))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize , serde :: Serialize))] # [cfg_attr (feature = "serde" , zerovec :: derive (Serialize , Deserialize , Debug))] pub struct CodePointInversionListAndStringList < 'data > { # [cfg_attr (feature = "serde" , serde (borrow))] # [zerovec :: varule (CodePointInversionListULE)] cp_inv_list : CodePointInversionList < 'data > , # [cfg_attr (feature = "serde" , serde (borrow))] str_list : VarZeroVec < 'data , str > , }
};
}
