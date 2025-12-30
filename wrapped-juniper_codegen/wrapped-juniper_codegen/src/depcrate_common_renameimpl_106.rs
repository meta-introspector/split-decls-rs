// Generated macro for impl_106 (impl)
macro_rules! Depcrate_common_renameimpl_106 {
() => {
// Module: crate::common::rename
// Provides: {"impl_106"}
// Dependencies: {}
impl FromStr for Policy { type Err = () ; fn from_str (rule : & str) -> Result < Self , Self :: Err > { match rule { "none" => Ok (Self :: None) , "camelCase" => Ok (Self :: CamelCase) , "snake_case" => Ok (Self :: SnakeCase) , "SCREAMING_SNAKE_CASE" => Ok (Self :: ScreamingSnakeCase) , _ => Err (()) , } } }
};
}
