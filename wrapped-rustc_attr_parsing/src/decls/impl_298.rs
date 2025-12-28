macro_rules! deps {
    () => {
        MetaItemParser!();
        MetaItemOrLitParser!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < 'a > MetaItemOrLitParser < 'a > { pub fn span (& self) -> Span { match self { MetaItemOrLitParser :: MetaItemParser (generic_meta_item_parser) => { generic_meta_item_parser . span () } MetaItemOrLitParser :: Lit (meta_item_lit) => meta_item_lit . span , MetaItemOrLitParser :: Err (span , _) => * span , } } pub fn lit (& self) -> Option < & MetaItemLit > { match self { MetaItemOrLitParser :: Lit (meta_item_lit) => Some (meta_item_lit) , _ => None , } } pub fn meta_item (& self) -> Option < & MetaItemParser < 'a > > { match self { MetaItemOrLitParser :: MetaItemParser (parser) => Some (parser) , _ => None , } } }
    };
}

impl_298!();