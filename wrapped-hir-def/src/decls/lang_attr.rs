macro_rules! deps {
    () => {
        AttrDefId!();
        DefDatabase!();
    };
}

macro_rules! lang_attr {
    () => {
        deps!();
        pub (crate) fn lang_attr (db : & dyn DefDatabase , item : AttrDefId) -> Option < LangItem > { db . attrs (item) . lang_item () }
    };
}

lang_attr!();