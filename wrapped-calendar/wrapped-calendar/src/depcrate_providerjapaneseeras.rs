// Generated macro for JapaneseEras (struct)
macro_rules! Depcrate_providerJapaneseEras {
() => {
// Module: crate::provider
// Provides: {"JapaneseEras"}
// Dependencies: {}
# [doc = " A data structure containing the necessary era data for constructing a"] # [doc = " [`Japanese`](crate::cal::Japanese) calendar object"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Debug , PartialEq , Clone , Default , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_calendar :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct JapaneseEras < 'data > { # [doc = " A map from era start dates to their era codes"] # [cfg_attr (feature = "serde" , serde (borrow))] pub dates_to_eras : ZeroVec < 'data , (EraStartDate , TinyStr16) > , }
};
}
