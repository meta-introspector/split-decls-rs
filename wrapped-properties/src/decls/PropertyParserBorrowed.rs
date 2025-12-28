macro_rules! PropertyParserBorrowed {
    () => {
        # [doc = " A borrowed wrapper around property value name-to-enum data, returned by"] # [doc = " [`PropertyParser::as_borrowed()`]. More efficient to query."] # [derive (Debug)] pub struct PropertyParserBorrowed < 'a , T > { map : & 'a PropertyValueNameToEnumMap < 'a > , markers : PhantomData < fn () -> T > , }
    };
}

PropertyParserBorrowed!()