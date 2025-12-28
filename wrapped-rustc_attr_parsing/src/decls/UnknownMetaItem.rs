macro_rules! UnknownMetaItem {
    () => {
        # [doc = " Error code: E0541"] pub (crate) struct UnknownMetaItem < 'a > { pub span : Span , pub item : String , pub expected : & 'a [& 'a str] , }
    };
}

UnknownMetaItem!();