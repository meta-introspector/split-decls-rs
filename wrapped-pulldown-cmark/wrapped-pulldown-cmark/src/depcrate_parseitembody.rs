// Generated macro for ItemBody (enum)
macro_rules! Depcrate_parseItemBody {
() => {
// Module: crate::parse
// Provides: {"ItemBody"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone , Copy , Default)] pub (crate) enum ItemBody { MaybeEmphasis (usize , bool , bool) , MaybeMath (bool , bool , u8) , MaybeSmartQuote (u8 , bool , bool) , MaybeCode (usize , bool) , MaybeHtml , MaybeLinkOpen , MaybeLinkClose (bool) , MaybeImage , Emphasis , Strong , Strikethrough , Superscript , Subscript , Math (CowIndex , bool) , Code (CowIndex) , Link (LinkIndex) , Image (LinkIndex) , FootnoteReference (CowIndex) , TaskListMarker (bool) , InlineHtml , OwnedInlineHtml (CowIndex) , SynthesizeText (CowIndex) , SynthesizeChar (char) , Html , Text { backslash_escaped : bool , } , SoftBreak , HardBreak (bool) , # [default] Root , Paragraph , TightParagraph , Rule , Heading (HeadingLevel , Option < HeadingIndex >) , FencedCodeBlock (CowIndex) , IndentCodeBlock , HtmlBlock , BlockQuote (Option < BlockQuoteKind >) , Container (u8 , ContainerKind , CowIndex) , List (bool , u8 , u64) , ListItem (usize) , FootnoteDefinition (CowIndex) , MetadataBlock (MetadataBlockKind) , DefinitionList (bool) , MaybeDefinitionListTitle , DefinitionListTitle , DefinitionListDefinition (usize) , Table (AlignmentIndex) , TableHead , TableRow , TableCell , }
};
}
