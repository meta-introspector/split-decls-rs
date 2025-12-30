// Generated macro for impl_273 (impl)
macro_rules! Depcrateimpl_273 {
() => {
// Module: crate
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'a > Event < 'a > { pub fn into_static (self) -> Event < 'static > { match self { Event :: Start (t) => Event :: Start (t . into_static ()) , Event :: End (e) => Event :: End (e) , Event :: Text (s) => Event :: Text (s . into_static ()) , Event :: Code (s) => Event :: Code (s . into_static ()) , Event :: InlineMath (s) => Event :: InlineMath (s . into_static ()) , Event :: DisplayMath (s) => Event :: DisplayMath (s . into_static ()) , Event :: Html (s) => Event :: Html (s . into_static ()) , Event :: InlineHtml (s) => Event :: InlineHtml (s . into_static ()) , Event :: FootnoteReference (s) => Event :: FootnoteReference (s . into_static ()) , Event :: SoftBreak => Event :: SoftBreak , Event :: HardBreak => Event :: HardBreak , Event :: Rule => Event :: Rule , Event :: TaskListMarker (b) => Event :: TaskListMarker (b) , } } }
};
}
