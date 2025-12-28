macro_rules! deps {
    () => {
        ShouldEmit!();
    };
}

macro_rules! MetaItemListParserContext {
    () => {
        deps!();
        struct MetaItemListParserContext < 'a , 'sess > { parser : & 'a mut Parser < 'sess > , should_emit : ShouldEmit , }
    };
}

MetaItemListParserContext!();