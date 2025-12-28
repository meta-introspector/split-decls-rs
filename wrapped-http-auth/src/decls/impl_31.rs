macro_rules! deps {
    () => {
        QopSet!();
        Qop!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl std :: ops :: BitAnd < Qop > for QopSet { type Output = bool ; fn bitand (self , rhs : Qop) -> Self :: Output { (self . 0 & (rhs as u8)) != 0 } }
    };
}

impl_31!();