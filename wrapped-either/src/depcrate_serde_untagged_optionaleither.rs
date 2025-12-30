// Generated macro for Either (enum)
macro_rules! Depcrate_serde_untagged_optionalEither {
() => {
// Module: crate::serde_untagged_optional
// Provides: {"Either"}
// Dependencies: {}
# [derive (Serialize , Deserialize)] # [serde (untagged)] enum Either < L , R > { Left (L) , Right (R) , }
};
}
