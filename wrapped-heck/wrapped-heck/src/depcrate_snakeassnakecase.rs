// Generated macro for AsSnakeCase (struct)
macro_rules! Depcrate_snakeAsSnakeCase {
() => {
// Module: crate::snake
// Provides: {"AsSnakeCase"}
// Dependencies: {}
# [doc = " This wrapper performs a snake case conversion in [`fmt::Display`]."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```"] # [doc = " use heck::AsSnakeCase;"] # [doc = ""] # [doc = " let sentence = \"We carry a new world here, in our hearts.\";"] # [doc = " assert_eq!(format!(\"{}\", AsSnakeCase(sentence)), \"we_carry_a_new_world_here_in_our_hearts\");"] # [doc = " ```"] pub struct AsSnakeCase < T : AsRef < str > > (pub T) ;
};
}
