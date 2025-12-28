macro_rules! deps {
    () => {
        ShouldPanic!();
    };
}

macro_rules! should_panic {
    () => {
        deps!();
        fn should_panic (cx : & ExtCtxt < '_ > , i : & ast :: Item) -> ShouldPanic { if let Some (Attribute :: Parsed (AttributeKind :: ShouldPanic { reason , .. })) = AttributeParser :: parse_limited (cx . sess , & i . attrs , sym :: should_panic , i . span , i . node_id () , None ,) { ShouldPanic :: Yes (reason) } else { ShouldPanic :: No } }
    };
}

should_panic!()