macro_rules! doc_comment_to_string {
    () => {
        pub fn doc_comment_to_string (comment_kind : CommentKind , attr_style : ast :: AttrStyle , data : Symbol ,) -> String { match (comment_kind , attr_style) { (CommentKind :: Line , ast :: AttrStyle :: Outer) => format ! ("///{data}") , (CommentKind :: Line , ast :: AttrStyle :: Inner) => format ! ("//!{data}") , (CommentKind :: Block , ast :: AttrStyle :: Outer) => format ! ("/**{data}*/") , (CommentKind :: Block , ast :: AttrStyle :: Inner) => format ! ("/*!{data}*/") , } }
    };
}

doc_comment_to_string!()