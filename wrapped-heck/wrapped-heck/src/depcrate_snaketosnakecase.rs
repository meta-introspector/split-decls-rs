// Generated macro for ToSnakeCase (trait)
macro_rules! Depcrate_snakeToSnakeCase {
() => {
// Module: crate::snake
// Provides: {"ToSnakeCase"}
// Dependencies: {}
# [doc = " This trait defines a snake case conversion."] # [doc = ""] # [doc = " In snake_case, word boundaries are indicated by underscores."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use heck::ToSnakeCase;"] # [doc = ""] # [doc = " let sentence = \"We carry a new world here, in our hearts.\";"] # [doc = " assert_eq!(sentence.to_snake_case(), \"we_carry_a_new_world_here_in_our_hearts\");"] # [doc = " ```"] pub trait ToSnakeCase : ToOwned { # [doc = " Convert this type to snake case."] fn to_snake_case (& self) -> Self :: Owned ; }
};
}
