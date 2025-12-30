// Generated macro for CodePointInversionList (struct)
macro_rules! Depcrate_codepointinvlist_cpinvlistCodePointInversionList {
() => {
// Module: crate::codepointinvlist::cpinvlist
// Provides: {"CodePointInversionList"}
// Dependencies: {}
# [doc = " A membership wrapper for [`CodePointInversionList`]."] # [doc = ""] # [doc = " Provides exposure to membership functions and constructors from serialized `CodePointSet`s (sets of code points)"] # [doc = " and predefined ranges."] # [zerovec :: make_varule (CodePointInversionListULE)] # [zerovec :: skip_derive (Ord)] # [zerovec :: derive (Debug)] # [derive (Debug , Eq , PartialEq , Clone , Yokeable , ZeroFrom)] # [cfg_attr (not (feature = "alloc") , zerovec :: skip_derive (ZeroMapKV , ToOwned))] pub struct CodePointInversionList < 'data > { inv_list : ZeroVec < 'data , PotentialCodePoint > , size : u32 , }
};
}
