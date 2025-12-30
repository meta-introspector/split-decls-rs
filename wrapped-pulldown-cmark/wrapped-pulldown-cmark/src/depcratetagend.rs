// Generated macro for TagEnd (enum)
macro_rules! DepcrateTagEnd {
() => {
// Module: crate
// Provides: {"TagEnd"}
// Dependencies: {}
# [doc = " The end of a `Tag`."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub enum TagEnd { Paragraph , Heading (HeadingLevel) , BlockQuote (Option < BlockQuoteKind >) , CodeBlock , ContainerBlock (ContainerKind) , HtmlBlock , # [doc = " A list, `true` for ordered lists."] List (bool) , Item , FootnoteDefinition , DefinitionList , DefinitionListTitle , DefinitionListDefinition , Table , TableHead , TableRow , TableCell , Emphasis , Strong , Strikethrough , Superscript , Subscript , Link , Image , MetadataBlock (MetadataBlockKind) , }
};
}
