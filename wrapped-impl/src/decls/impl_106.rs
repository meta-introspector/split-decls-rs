macro_rules! deps {
    () => {
        Display!();
        MemberUnraw!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl Display for MemberUnraw { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match self { MemberUnraw :: Named (this) => Display :: fmt (this , formatter) , MemberUnraw :: Unnamed (this) => Display :: fmt (& this . index , formatter) , } } }
    };
}

impl_106!();