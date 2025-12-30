// Generated macro for title_block (function)
macro_rules! Depcratetitle_block {
() => {
// Module: crate
// Provides: {"title_block"}
// Dependencies: {}
fn title_block (title : & str) -> Block < '_ > { let title = Line :: from (title) . centered () ; Block :: new () . borders (Borders :: NONE) . padding (Padding :: vertical (1)) . title (title) . fg (CUSTOM_LABEL_COLOR) }
};
}
