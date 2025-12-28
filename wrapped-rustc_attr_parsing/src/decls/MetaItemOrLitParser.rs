macro_rules! deps {
    () => {
        MetaItemParser!();
    };
}

macro_rules! MetaItemOrLitParser {
    () => {
        deps!();
        # [doc = " Inside lists, values could be either literals, or more deeply nested meta items."] # [doc = " This enum represents that."] # [doc = ""] # [doc = " Choose which one you want using the provided methods."] # [derive (Debug , Clone)] pub enum MetaItemOrLitParser < 'a > { MetaItemParser (MetaItemParser < 'a >) , Lit (MetaItemLit) , Err (Span , ErrorGuaranteed) , }
    };
}

MetaItemOrLitParser!()