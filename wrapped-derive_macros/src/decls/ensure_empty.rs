macro_rules! ensure_empty {
    () => {
        pub (crate) fn ensure_empty (ctxt : & str , attrs : & [Attribute]) { for (value_key , _) in attrs . iter () . filter_map (| attr | sval_attr (ctxt , attr)) . flatten () { panic ! ("unsupported attribute `{}` on {}" , quote ! (# value_key) , ctxt) ; } }
    };
}

ensure_empty!();