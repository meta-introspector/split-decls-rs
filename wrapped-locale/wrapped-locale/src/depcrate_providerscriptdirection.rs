// Generated macro for ScriptDirection (struct)
macro_rules! Depcrate_providerScriptDirection {
() => {
// Module: crate::provider
// Provides: {"ScriptDirection"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_locale :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [doc = " This directionality data is used to determine the script directionality of a locale."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [yoke (prove_covariance_manually)] pub struct ScriptDirection < 'data > { # [doc = " Scripts in right-to-left direction."] # [cfg_attr (feature = "serde" , serde (borrow))] pub rtl : ZeroVec < 'data , UnvalidatedScript > , # [doc = " Scripts in left-to-right direction."] # [cfg_attr (feature = "serde" , serde (borrow))] pub ltr : ZeroVec < 'data , UnvalidatedScript > , }
};
}
