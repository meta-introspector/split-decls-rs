// Generated macro for Either (enum)
macro_rules! Depcrate_serde_untaggedEither {
() => {
// Module: crate::serde_untagged
// Provides: {"Either"}
// Dependencies: {}
# [derive (serde :: Serialize , serde :: Deserialize)] # [serde (untagged)] enum Either < L , R > { Left (L) , Right (R) , }
};
}
