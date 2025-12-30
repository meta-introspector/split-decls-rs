// Generated macro for example_height (function)
macro_rules! Depcrateexample_height {
() => {
// Module: crate
// Provides: {"example_height"}
// Dependencies: {}
# [doc = " The height of all examples combined"] # [doc = ""] # [doc = " Each may or may not have a title so we need to account for that."] fn example_height () -> u16 { EXAMPLE_DATA . iter () . map (| (desc , _) | get_description_height (desc) + 4) . sum () }
};
}
