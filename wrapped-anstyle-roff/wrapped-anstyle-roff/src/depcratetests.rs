// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use anstyle :: RgbColor ; # [test] fn test_to_hex () { assert_eq ! (to_hex (& RgbColor (0 , 0 , 0)) . as_str () , "#000000") ; assert_eq ! (to_hex (& RgbColor (255 , 0 , 0)) . as_str () , "#ff0000") ; assert_eq ! (to_hex (& RgbColor (0 , 255 , 0)) . as_str () , "#00ff00") ; assert_eq ! (to_hex (& RgbColor (0 , 0 , 255)) . as_str () , "#0000ff") ; } }
};
}
