// Generated macro for title_block (function)
macro_rules! Depcratetitle_block {
() => {
// Module: crate
// Provides: {"title_block"}
// Dependencies: {}
fn title_block (title : String) -> Block < 'static > { Block :: new () . borders (Borders :: TOP) . title_alignment (Alignment :: Center) . border_style (Style :: new () . dark_gray ()) . title_style (Style :: reset ()) . title (title) }
};
}
