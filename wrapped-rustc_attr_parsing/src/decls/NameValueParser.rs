macro_rules! NameValueParser {
    () => {
        # [derive (Clone)] pub struct NameValueParser { pub eq_span : Span , value : MetaItemLit , pub value_span : Span , }
    };
}

NameValueParser!()