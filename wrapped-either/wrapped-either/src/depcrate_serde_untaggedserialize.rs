// Generated macro for serialize (function)
macro_rules! Depcrate_serde_untaggedserialize {
() => {
// Module: crate::serde_untagged
// Provides: {"serialize"}
// Dependencies: {}
pub fn serialize < L , R , S > (this : & super :: Either < L , R > , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , L : Serialize , R : Serialize , { let untagged = match this { super :: Either :: Left (left) => Either :: Left (left) , super :: Either :: Right (right) => Either :: Right (right) , } ; untagged . serialize (serializer) }
};
}
