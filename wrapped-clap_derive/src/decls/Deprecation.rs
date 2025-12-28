macro_rules! Deprecation {
    () => {
        # [derive (Clone)] pub (crate) struct Deprecation { pub (crate) span : Span , pub (crate) id : & 'static str , pub (crate) version : & 'static str , pub (crate) description : String , }
    };
}

Deprecation!()