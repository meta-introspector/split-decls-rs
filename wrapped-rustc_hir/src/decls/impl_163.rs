macro_rules! deps {
    () => {
        Attribute!();
        AttrItem!();
        AttrArgs!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl Attribute { pub fn get_normal_item (& self) -> & AttrItem { match & self { Attribute :: Unparsed (normal) => & normal , _ => panic ! ("unexpected parsed attribute") , } } pub fn unwrap_normal_item (self) -> AttrItem { match self { Attribute :: Unparsed (normal) => * normal , _ => panic ! ("unexpected parsed attribute") , } } pub fn value_lit (& self) -> Option < & MetaItemLit > { match & self { Attribute :: Unparsed (n) => match n . as_ref () { AttrItem { args : AttrArgs :: Eq { eq_span : _ , expr } , .. } => Some (expr) , _ => None , } , _ => None , } } pub fn is_parsed_attr (& self) -> bool { match self { Attribute :: Parsed (_) => true , Attribute :: Unparsed (_) => false , } } }
    };
}

impl_163!()