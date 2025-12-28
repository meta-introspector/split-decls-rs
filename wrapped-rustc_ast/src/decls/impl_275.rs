macro_rules! deps {
    () => {
        Attribute!();
        AttrItem!();
        AttrKind!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl Attribute { pub fn get_normal_item (& self) -> & AttrItem { match & self . kind { AttrKind :: Normal (normal) => & normal . item , AttrKind :: DocComment (..) => panic ! ("unexpected doc comment") , } } pub fn unwrap_normal_item (self) -> AttrItem { match self . kind { AttrKind :: Normal (normal) => normal . item , AttrKind :: DocComment (..) => panic ! ("unexpected doc comment") , } } }
    };
}

impl_275!();