macro_rules! deps {
    () => {
        AttributeExt!();
        CommentKind!();
        MetaItemInner!();
        Attribute!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl Attribute { pub fn id (& self) -> AttrId { AttributeExt :: id (self) } pub fn name (& self) -> Option < Symbol > { AttributeExt :: name (self) } pub fn meta_item_list (& self) -> Option < ThinVec < MetaItemInner > > { AttributeExt :: meta_item_list (self) } pub fn value_str (& self) -> Option < Symbol > { AttributeExt :: value_str (self) } pub fn value_span (& self) -> Option < Span > { AttributeExt :: value_span (self) } pub fn ident (& self) -> Option < Ident > { AttributeExt :: ident (self) } pub fn path_matches (& self , name : & [Symbol]) -> bool { AttributeExt :: path_matches (self , name) } pub fn is_doc_comment (& self) -> bool { AttributeExt :: is_doc_comment (self) } # [inline] pub fn has_name (& self , name : Symbol) -> bool { AttributeExt :: has_name (self , name) } # [inline] pub fn has_any_name (& self , names : & [Symbol]) -> bool { AttributeExt :: has_any_name (self , names) } pub fn span (& self) -> Span { AttributeExt :: span (self) } pub fn is_word (& self) -> bool { AttributeExt :: is_word (self) } pub fn path (& self) -> SmallVec < [Symbol ; 1] > { AttributeExt :: path (self) } pub fn ident_path (& self) -> Option < SmallVec < [Ident ; 1] > > { AttributeExt :: ident_path (self) } pub fn doc_str (& self) -> Option < Symbol > { AttributeExt :: doc_str (self) } pub fn is_proc_macro_attr (& self) -> bool { AttributeExt :: is_proc_macro_attr (self) } pub fn doc_str_and_comment_kind (& self) -> Option < (Symbol , CommentKind) > { AttributeExt :: doc_str_and_comment_kind (self) } }
    };
}

impl_295!()