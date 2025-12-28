macro_rules! deps {
    () => {
        MetaItemOrLitParser!();
    };
}

macro_rules! MetaItemListParser {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct MetaItemListParser < 'a > { sub_parsers : ThinVec < MetaItemOrLitParser < 'a > > , pub span : Span , }
    };
}

MetaItemListParser!();