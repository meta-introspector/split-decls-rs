// Generated macro for EraStartDate (struct)
macro_rules! Depcrate_providerEraStartDate {
() => {
// Module: crate::provider
// Provides: {"EraStartDate"}
// Dependencies: {}
# [doc = " The date at which an era started"] # [doc = ""] # [doc = " The order of fields in this struct is important!"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [zerovec :: make_ule (EraStartDateULE)] # [derive (Copy , Clone , PartialEq , PartialOrd , Eq , Ord , Hash , Debug , yoke :: Yokeable , zerofrom :: ZeroFrom ,)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_calendar :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (not (feature = "alloc") , zerovec :: skip_derive (ZeroMapKV))] pub struct EraStartDate { # [doc = " The year the era started in"] pub year : i32 , # [doc = " The month the era started in"] pub month : u8 , # [doc = " The day the era started in"] pub day : u8 , }
};
}
