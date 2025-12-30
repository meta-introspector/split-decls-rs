// Generated macro for ScrollUpInRegion (struct)
macro_rules! DepcrateScrollUpInRegion {
() => {
// Module: crate
// Provides: {"ScrollUpInRegion"}
// Dependencies: {}
# [doc = " A command that scrolls the terminal screen a given number of rows up in a specific scrolling"] # [doc = " region."] # [doc = ""] # [doc = " This will hopefully be replaced by a struct in crossterm proper. There are two outstanding"] # [doc = " crossterm PRs that will address this:"] # [doc = "   - [918](https://github.com/crossterm-rs/crossterm/pull/918)"] # [doc = "   - [923](https://github.com/crossterm-rs/crossterm/pull/923)"] # [cfg (feature = "scrolling-regions")] # [derive (Debug , Clone , Copy , PartialEq , Eq)] struct ScrollUpInRegion { # [doc = " The first row of the scrolling region."] pub first_row : u16 , # [doc = " The last row of the scrolling region."] pub last_row : u16 , # [doc = " The number of lines to scroll up by."] pub lines_to_scroll : u16 , }
};
}
