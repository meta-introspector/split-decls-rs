macro_rules! deps {
    () => {
        ReprInfo!();
        IdentListAttribute!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl ReprInfo { pub fn compute (attrs : & [Attribute]) -> Self { let mut info = ReprInfo :: default () ; for attr in attrs . iter () . filter (| a | a . path () . is_ident ("repr")) { if let Ok (pieces) = attr . parse_args :: < IdentListAttribute > () { for piece in pieces . idents . iter () { if piece == "C" || piece == "c" { info . c = true ; } else if piece == "transparent" { info . transparent = true ; } else if piece == "packed" { info . packed = true ; } else if piece == "u8" { info . u8 = true ; } } } } info } pub fn cpacked_or_transparent (self) -> bool { (self . c && self . packed) || self . transparent } }
    };
}

impl_22!()