macro_rules! deps {
    () => {
        CommentKind!();
        AttrStyle!();
        Attribute!();
        AttrIdGenerator!();
        AttrKind!();
    };
}

macro_rules! mk_doc_comment {
    () => {
        deps!();
        pub fn mk_doc_comment (g : & AttrIdGenerator , comment_kind : CommentKind , style : AttrStyle , data : Symbol , span : Span ,) -> Attribute { Attribute { kind : AttrKind :: DocComment (comment_kind , data) , id : g . mk_attr_id () , style , span } }
    };
}

mk_doc_comment!()