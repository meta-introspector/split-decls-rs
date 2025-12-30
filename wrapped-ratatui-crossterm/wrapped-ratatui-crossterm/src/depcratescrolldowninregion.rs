// Generated macro for ScrollDownInRegion (struct)
macro_rules! DepcrateScrollDownInRegion {
() => {
// Module: crate
// Provides: {"ScrollDownInRegion"}
// Dependencies: {}
# [doc = " A command that scrolls the terminal screen a given number of rows down in a specific scrolling"] # [doc = " region."] # [doc = ""] # [doc = " This will hopefully be replaced by a struct in crossterm proper. There are two outstanding"] # [doc = " crossterm PRs that will address this:"] # [doc = "   - [918](https://github.com/crossterm-rs/crossterm/pull/918)"] # [doc = "   - [923](https://github.com/crossterm-rs/crossterm/pull/923)"] # [cfg (feature = "scrolling-regions")] # [derive (Debug , Clone , Copy , PartialEq , Eq)] struct ScrollDownInRegion { # [doc = " The first row of the scrolling region."] pub first_row : u16 , # [doc = " The last row of the scrolling region."] pub last_row : u16 , # [doc = " The number of lines to scroll down by."] pub lines_to_scroll : u16 , }
};
}
