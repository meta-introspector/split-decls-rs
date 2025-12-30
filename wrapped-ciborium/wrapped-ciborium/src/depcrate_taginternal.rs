// Generated macro for Internal (enum)
macro_rules! Depcrate_tagInternal {
() => {
// Module: crate::tag
// Provides: {"Internal"}
// Dependencies: {}
# [derive (Deserialize , Serialize)] # [serde (rename = "@@TAG@@")] enum Internal < T > { # [serde (rename = "@@UNTAGGED@@")] Untagged (T) , # [serde (rename = "@@TAGGED@@")] Tagged (u64 , T) , }
};
}
