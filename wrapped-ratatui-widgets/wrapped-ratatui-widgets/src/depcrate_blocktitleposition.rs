// Generated macro for TitlePosition (enum)
macro_rules! Depcrate_blockTitlePosition {
() => {
// Module: crate::block
// Provides: {"TitlePosition"}
// Dependencies: {}
# [doc = " Defines the position of the title."] # [doc = ""] # [doc = " The title can be positioned on top or at the bottom of the block."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use ratatui::widgets::{Block, TitlePosition};"] # [doc = ""] # [doc = " Block::bordered()"] # [doc = "     .title_position(TitlePosition::Top)"] # [doc = "     .title(\"Top Title\");"] # [doc = " Block::bordered()"] # [doc = "     .title_position(TitlePosition::Bottom)"] # [doc = "     .title(\"Bottom Title\");"] # [doc = " ```"] # [derive (Debug , Default , Display , EnumString , Clone , Copy , PartialEq , Eq , Hash)] pub enum TitlePosition { # [doc = " Position the title at the top of the block."] # [default] Top , # [doc = " Position the title at the bottom of the block."] Bottom , }
};
}
