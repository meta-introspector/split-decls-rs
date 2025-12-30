// Generated macro for CaseMap (struct)
macro_rules! Depcrate_providerCaseMap {
() => {
// Module: crate::provider
// Provides: {"CaseMap"}
// Dependencies: {}
# [doc = " This type contains all of the casemapping data"] # [doc = ""] # [doc = " The methods in the provider module are primarily about accessing its data,"] # [doc = " however the full algorithms are also implemented as methods on this type in"] # [doc = " the `internals` module of this crate."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_casemap :: provider))] # [yoke (prove_covariance_manually)] # [doc = " CaseMapper provides low-level access to the data necessary to"] # [doc = " convert characters and strings to upper, lower, or title case."] pub struct CaseMap < 'data > { # [doc = " Case mapping data"] pub trie : CodePointTrie < 'data , CaseMapData > , # [doc = " Exceptions to the case mapping data"] pub exceptions : CaseMapExceptions < 'data > , }
};
}
