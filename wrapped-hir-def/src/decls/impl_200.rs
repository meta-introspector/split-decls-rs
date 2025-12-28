macro_rules! deps {
    () => {
        AttrDefId!();
        LangItems!();
        DefDatabase!();
        LangItemTarget!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl LangItems { pub fn target (& self , item : LangItem) -> Option < LangItemTarget > { self . items . get (& item) . copied () } fn collect_lang_item < T > (& mut self , db : & dyn DefDatabase , item : T , constructor : fn (T) -> LangItemTarget ,) where T : Into < AttrDefId > + Copy , { let _p = tracing :: info_span ! ("collect_lang_item") . entered () ; if let Some (lang_item) = lang_attr (db , item . into ()) { self . items . entry (lang_item) . or_insert_with (| | constructor (item)) ; } } }
    };
}

impl_200!()