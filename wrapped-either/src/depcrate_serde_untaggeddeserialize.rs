// Generated macro for deserialize (function)
macro_rules! Depcrate_serde_untaggeddeserialize {
() => {
// Module: crate::serde_untagged
// Provides: {"deserialize"}
// Dependencies: {}
pub fn deserialize < 'de , L , R , D > (deserializer : D) -> Result < super :: Either < L , R > , D :: Error > where D : Deserializer < 'de > , L : Deserialize < 'de > , R : Deserialize < 'de > , { match Either :: deserialize (deserializer) { Ok (Either :: Left (left)) => Ok (super :: Either :: Left (left)) , Ok (Either :: Right (right)) => Ok (super :: Either :: Right (right)) , Err (error) => Err (error) , } }
};
}
