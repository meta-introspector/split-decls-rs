// Generated macro for InlayHintLabelPart (struct)
macro_rules! Depcrate_inlay_hintsInlayHintLabelPart {
() => {
// Module: crate::inlay_hints
// Provides: {"InlayHintLabelPart"}
// Dependencies: {}
# [derive (UpmapFromRaFixture)] pub struct InlayHintLabelPart { pub text : String , # [doc = " Source location represented by this label part. The client will use this to fetch the part's"] # [doc = " hover tooltip, and Ctrl+Clicking the label part will navigate to the definition the location"] # [doc = " refers to (not necessarily the location itself)."] # [doc = " When setting this, no tooltip must be set on the containing hint, or VS Code will display"] # [doc = " them both."] pub linked_location : Option < LazyProperty < FileRange > > , # [doc = " The tooltip to show when hovering over the inlay hint, this may invoke other actions like"] # [doc = " hover requests to show."] pub tooltip : Option < LazyProperty < InlayTooltip > > , }
};
}
