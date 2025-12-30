// Generated macro for AsShoutySnakeCase (struct)
macro_rules! Depcrate_shouty_snakeAsShoutySnakeCase {
() => {
// Module: crate::shouty_snake
// Provides: {"AsShoutySnakeCase"}
// Dependencies: {}
# [doc = " This wrapper performs a shouty snake  case conversion in [`fmt::Display`]."] # [doc = ""] # [doc = " ## Example:"] # [doc = ""] # [doc = " ```"] # [doc = " use heck::AsShoutySnakeCase;"] # [doc = ""] # [doc = " let sentence = \"That world is growing in this minute.\";"] # [doc = " assert_eq!(format!(\"{}\", AsShoutySnakeCase(sentence)), \"THAT_WORLD_IS_GROWING_IN_THIS_MINUTE\");"] # [doc = " ```"] pub struct AsShoutySnakeCase < T : AsRef < str > > (pub T) ;
};
}
