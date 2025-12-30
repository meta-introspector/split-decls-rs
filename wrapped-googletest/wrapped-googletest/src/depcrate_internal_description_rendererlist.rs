// Generated macro for List (struct)
macro_rules! Depcrate_internal_description_rendererList {
() => {
// Module: crate::internal::description_renderer
// Provides: {"List"}
// Dependencies: {}
# [doc = " A list of [`Block`] possibly rendered with a [`Decoration`]."] # [doc = ""] # [doc = " This is the top-level renderable component, corresponding to the description"] # [doc = " or match explanation of a single matcher."] # [doc = ""] # [doc = " The constituent [`Block`] of a `List` can be decorated with either bullets"] # [doc = " (`* `) or enumeration (`0. `, `1. `, ...). This is controlled via the"] # [doc = " methods [`List::bullet_list`] and [`List::enumerate`]. By default, there is"] # [doc = " no decoration."] # [doc = ""] # [doc = " A `List` can be constructed as follows:"] # [doc = ""] # [doc = "   * [`Default::default()`] constructs an empty `List`."] # [doc = "   * [`Iterator::collect()`] on an [`Iterator`] of [`Block`]."] # [doc = "   * [`Iterator::collect()`] on an [`Iterator`] of `String`, which produces a"] # [doc = "     [`Block::Literal`] for each `String`."] # [doc = "   * [`Iterator::collect()`] on an [`Iterator`] of `List`, which produces a"] # [doc = "     [`Block::Nested`] for each `List`."] # [derive (Debug , Default)] pub (crate) struct List (Vec < Block > , Decoration) ;
};
}
