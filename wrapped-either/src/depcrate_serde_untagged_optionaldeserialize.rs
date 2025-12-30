// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_untagged_optionaldeserialize {
() => {
// Module: crate::serde_untagged_optional
// Provides: {"deserialize"}
// Dependencies: {}
pub fn deserialize < 'de , L , R , D > (deserializer : D) -> Result < Option < super :: Either < L , R > > , D :: Error > where D : Deserializer < 'de > , L : Deserialize < 'de > , R : Deserialize < 'de > , { match Option :: deserialize (deserializer) { Ok (Some (Either :: Left (left))) => Ok (Some (super :: Either :: Left (left))) , Ok (Some (Either :: Right (right))) => Ok (Some (super :: Either :: Right (right))) , Ok (None) => Ok (None) , Err (error) => Err (error) , } }
};
}
