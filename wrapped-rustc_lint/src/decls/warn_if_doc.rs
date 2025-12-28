macro_rules! deps {
    () => {
        BuiltinUnusedDocComment!();
        EarlyContext!();
        BuiltinUnusedDocCommentSub!();
    };
}

macro_rules! warn_if_doc {
    () => {
        deps!();
        fn warn_if_doc (cx : & EarlyContext < '_ > , node_span : Span , node_kind : & str , attrs : & [ast :: Attribute]) { use rustc_ast :: token :: CommentKind ; let mut attrs = attrs . iter () . peekable () ; let mut sugared_span : Option < Span > = None ; while let Some (attr) = attrs . next () { let is_doc_comment = attr . is_doc_comment () ; if is_doc_comment { sugared_span = Some (sugared_span . map_or (attr . span , | span | span . with_hi (attr . span . hi ()))) ; } if attrs . peek () . is_some_and (| next_attr | next_attr . is_doc_comment ()) { continue ; } let span = sugared_span . take () . unwrap_or (attr . span) ; if is_doc_comment || attr . has_name (sym :: doc) { let sub = match attr . kind { AttrKind :: DocComment (CommentKind :: Line , _) | AttrKind :: Normal (..) => { BuiltinUnusedDocCommentSub :: PlainHelp } AttrKind :: DocComment (CommentKind :: Block , _) => { BuiltinUnusedDocCommentSub :: BlockHelp } } ; cx . emit_span_lint (UNUSED_DOC_COMMENTS , span , BuiltinUnusedDocComment { kind : node_kind , label : node_span , sub } ,) ; } } }
    };
}

warn_if_doc!()