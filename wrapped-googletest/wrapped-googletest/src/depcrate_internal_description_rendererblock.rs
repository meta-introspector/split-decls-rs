// Generated macro for Block (enum)
macro_rules! Depcrate_internal_description_rendererBlock {
() => {
// Module: crate::internal::description_renderer
// Provides: {"Block"}
// Dependencies: {}
# [doc = " A sequence of [`Fragment`] or a nested [`List`]."] # [doc = ""] # [doc = " This may be rendered with a prefix specified by the [`Decoration`] of the"] # [doc = " containing [`List`]. In this case, all lines are indented to align with the"] # [doc = " first character of the first line of the block."] # [derive (Debug)] enum Block { # [doc = " A block of text."] # [doc = ""] # [doc = " Each constituent [`Fragment`] contains one line of text. The lines are"] # [doc = " indented uniformly to the current indentation of this block when"] # [doc = " rendered."] Literal (Vec < Fragment >) , # [doc = " A nested [`List`]."] # [doc = ""] # [doc = " The [`List`] is rendered recursively at the next level of indentation."] Nested (List) , }
};
}
