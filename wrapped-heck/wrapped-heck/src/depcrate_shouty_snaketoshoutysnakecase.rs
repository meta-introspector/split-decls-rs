// Generated macro for ToShoutySnakeCase (trait)
macro_rules! Depcrate_shouty_snakeToShoutySnakeCase {
() => {
// Module: crate::shouty_snake
// Provides: {"ToShoutySnakeCase"}
// Dependencies: {}
# [doc = " This trait defines a shouty snake case conversion."] # [doc = ""] # [doc = " In SHOUTY_SNAKE_CASE, word boundaries are indicated by underscores and all"] # [doc = " words are in uppercase."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use heck::ToShoutySnakeCase;"] # [doc = ""] # [doc = " let sentence = \"That world is growing in this minute.\";"] # [doc = " assert_eq!(sentence.to_shouty_snake_case(), \"THAT_WORLD_IS_GROWING_IN_THIS_MINUTE\");"] # [doc = " ```"] pub trait ToShoutySnakeCase : ToOwned { # [doc = " Convert this type to shouty snake case."] fn to_shouty_snake_case (& self) -> Self :: Owned ; }
};
}
