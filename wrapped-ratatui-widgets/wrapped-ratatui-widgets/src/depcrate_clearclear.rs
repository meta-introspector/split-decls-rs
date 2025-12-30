// Generated macro for Clear (struct)
macro_rules! Depcrate_clearClear {
() => {
// Module: crate::clear
// Provides: {"Clear"}
// Dependencies: {}
# [doc = " A widget to clear/reset a certain area to allow overdrawing (e.g. for popups)."] # [doc = ""] # [doc = " This widget **cannot be used to clear the terminal on the first render** as `ratatui` assumes"] # [doc = " the render area is empty. Use `Terminal::clear` instead."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::Frame;"] # [doc = " use ratatui::layout::Rect;"] # [doc = " use ratatui::widgets::{Block, Clear};"] # [doc = ""] # [doc = " fn draw_on_clear(f: &mut Frame, area: Rect) {"] # [doc = "     let block = Block::bordered().title(\"Block\");"] # [doc = "     f.render_widget(Clear, area); // <- this will clear/reset the area first"] # [doc = "     f.render_widget(block, area); // now render the block widget"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Popup Example"] # [doc = ""] # [doc = " For a more complete example how to utilize `Clear` to realize popups see"] # [doc = " the example `examples/popup.rs`"] # [derive (Debug , Default , Clone , Eq , PartialEq , Hash)] pub struct Clear ;
};
}
