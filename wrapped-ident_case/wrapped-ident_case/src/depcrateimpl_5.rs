// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl FromStr for RenameRule { type Err = () ; fn from_str (rename_all_str : & str) -> Result < Self , Self :: Err > { match rename_all_str { "lowercase" => Ok (LowerCase) , "PascalCase" => Ok (PascalCase) , "camelCase" => Ok (CamelCase) , "snake_case" => Ok (SnakeCase) , "SCREAMING_SNAKE_CASE" => Ok (ScreamingSnakeCase) , "kebab-case" => Ok (KebabCase) , _ => Err (()) , } } }
};
}
