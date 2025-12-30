// Generated macro for body_to_tag_end (function)
macro_rules! Depcrate_parsebody_to_tag_end {
() => {
// Module: crate::parse
// Provides: {"body_to_tag_end"}
// Dependencies: {}
fn body_to_tag_end (body : & ItemBody) -> TagEnd { match * body { ItemBody :: Paragraph => TagEnd :: Paragraph , ItemBody :: Emphasis => TagEnd :: Emphasis , ItemBody :: Superscript => TagEnd :: Superscript , ItemBody :: Subscript => TagEnd :: Subscript , ItemBody :: Strong => TagEnd :: Strong , ItemBody :: Strikethrough => TagEnd :: Strikethrough , ItemBody :: Link (..) => TagEnd :: Link , ItemBody :: Image (..) => TagEnd :: Image , ItemBody :: Heading (level , _) => TagEnd :: Heading (level) , ItemBody :: IndentCodeBlock | ItemBody :: FencedCodeBlock (..) => TagEnd :: CodeBlock , ItemBody :: Container (_ , kind , _) => TagEnd :: ContainerBlock (kind) , ItemBody :: BlockQuote (kind) => TagEnd :: BlockQuote (kind) , ItemBody :: HtmlBlock => TagEnd :: HtmlBlock , ItemBody :: List (_ , c , _) => { let is_ordered = c == b'.' || c == b')' ; TagEnd :: List (is_ordered) } ItemBody :: ListItem (_) => TagEnd :: Item , ItemBody :: TableHead => TagEnd :: TableHead , ItemBody :: TableCell => TagEnd :: TableCell , ItemBody :: TableRow => TagEnd :: TableRow , ItemBody :: Table (..) => TagEnd :: Table , ItemBody :: FootnoteDefinition (..) => TagEnd :: FootnoteDefinition , ItemBody :: MetadataBlock (kind) => TagEnd :: MetadataBlock (kind) , ItemBody :: DefinitionList (_) => TagEnd :: DefinitionList , ItemBody :: DefinitionListTitle => TagEnd :: DefinitionListTitle , ItemBody :: DefinitionListDefinition (_) => TagEnd :: DefinitionListDefinition , _ => panic ! ("unexpected item body {:?}" , body) , } }
};
}
