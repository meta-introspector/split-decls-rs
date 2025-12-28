macro_rules! assoc_tag_str {
    () => {
        pub (crate) fn assoc_tag_str (assoc_tag : ty :: AssocTag) -> & 'static str { match assoc_tag { ty :: AssocTag :: Fn => "function" , ty :: AssocTag :: Const => "constant" , ty :: AssocTag :: Type => "type" , } }
    };
}

assoc_tag_str!();