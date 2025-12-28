macro_rules! deps {
    () => {
        MagicAttrName!();
        Sp!();
        AttrValue!();
        AttrKind!();
    };
}

macro_rules! ClapAttr {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct ClapAttr { pub (crate) kind : Sp < AttrKind > , pub (crate) name : Ident , pub (crate) magic : Option < MagicAttrName > , pub (crate) value : Option < AttrValue > , }
    };
}

ClapAttr!();