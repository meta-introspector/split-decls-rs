// Generated macro for ExemplarCharactersData (struct)
macro_rules! Depcrate_providerExemplarCharactersData {
() => {
// Module: crate::provider
// Provides: {"ExemplarCharactersData"}
// Dependencies: {}
# [doc = " A set of characters and strings which share a particular property value."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , Eq , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake) , databake (path = icu_locale :: provider) ,)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct ExemplarCharactersData < 'data > (# [cfg_attr (feature = "serde" , serde (borrow))] pub CodePointInversionListAndStringList < 'data > ,) ;
};
}
