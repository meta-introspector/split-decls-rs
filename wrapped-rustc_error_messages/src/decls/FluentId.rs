macro_rules! FluentId {
    () => {
        # [doc = " Identifier for the Fluent message/attribute corresponding to a diagnostic message."] type FluentId = Cow < 'static , str > ;
    };
}

FluentId!()