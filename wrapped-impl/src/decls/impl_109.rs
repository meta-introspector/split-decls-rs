macro_rules! deps {
    () => {
        MemberUnraw!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl PartialEq < str > for MemberUnraw { fn eq (& self , other : & str) -> bool { match self { MemberUnraw :: Named (this) => this == other , MemberUnraw :: Unnamed (_) => false , } } }
    };
}

impl_109!();