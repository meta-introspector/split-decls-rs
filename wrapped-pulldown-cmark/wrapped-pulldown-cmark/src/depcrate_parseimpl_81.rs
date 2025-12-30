// Generated macro for impl_81 (impl)
macro_rules! Depcrate_parseimpl_81 {
() => {
// Module: crate::parse
// Provides: {"impl_81"}
// Dependencies: {}
impl ItemBody { fn is_maybe_inline (& self) -> bool { use ItemBody :: * ; matches ! (* self , MaybeEmphasis (..) | MaybeMath (..) | MaybeSmartQuote (..) | MaybeCode (..) | MaybeHtml | MaybeLinkOpen | MaybeLinkClose (..) | MaybeImage) } fn is_inline (& self) -> bool { use ItemBody :: * ; matches ! (* self , MaybeEmphasis (..) | MaybeMath (..) | MaybeSmartQuote (..) | MaybeCode (..) | MaybeHtml | MaybeLinkOpen | MaybeLinkClose (..) | MaybeImage | Emphasis | Strong | Strikethrough | Math (..) | Code (..) | Link (..) | Image (..) | FootnoteReference (..) | TaskListMarker (..) | InlineHtml | OwnedInlineHtml (..) | SynthesizeText (..) | SynthesizeChar (..) | Html | Text { .. } | SoftBreak | HardBreak (..)) } }
};
}
