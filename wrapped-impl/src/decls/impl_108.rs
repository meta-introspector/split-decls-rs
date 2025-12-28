macro_rules! deps {
    () => {
        MemberUnraw!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl PartialEq for MemberUnraw { fn eq (& self , other : & Self) -> bool { match (self , other) { (MemberUnraw :: Named (this) , MemberUnraw :: Named (other)) => this == other , (MemberUnraw :: Unnamed (this) , MemberUnraw :: Unnamed (other)) => this == other , _ => false , } } }
    };
}

impl_108!()