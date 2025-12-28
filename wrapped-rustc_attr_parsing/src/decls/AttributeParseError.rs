macro_rules! deps {
    () => {
        AttributeParseErrorReason!();
    };
}

macro_rules! AttributeParseError {
    () => {
        deps!();
        pub (crate) struct AttributeParseError < 'a > { pub (crate) span : Span , pub (crate) attr_span : Span , pub (crate) attr_style : AttrStyle , pub (crate) template : AttributeTemplate , pub (crate) attribute : AttrPath , pub (crate) reason : AttributeParseErrorReason < 'a > , }
    };
}

AttributeParseError!();