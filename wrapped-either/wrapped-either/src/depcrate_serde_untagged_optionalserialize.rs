// Generated macro for serialize (function)
macro_rules! Depcrate_serde_untagged_optionalserialize {
() => {
// Module: crate::serde_untagged_optional
// Provides: {"serialize"}
// Dependencies: {}
pub fn serialize < L , R , S > (this : & Option < super :: Either < L , R > > , serializer : S ,) -> Result < S :: Ok , S :: Error > where S : Serializer , L : Serialize , R : Serialize , { let untagged = match this { Some (super :: Either :: Left (left)) => Some (Either :: Left (left)) , Some (super :: Either :: Right (right)) => Some (Either :: Right (right)) , None => None , } ; untagged . serialize (serializer) }
};
}
