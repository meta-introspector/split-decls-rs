macro_rules! deps {
    () => {
        TargetRef!();
        Target!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a > PartialEq < TargetRef < 'a > > for Target { fn eq (& self , other : & TargetRef < 'a >) -> bool { match (self , other) { (Target :: Object (lhs) , TargetRef :: Object (rhs)) => lhs == rhs , (Target :: Symbolic (lhs) , TargetRef :: Symbolic (rhs)) => lhs . as_bstr () == rhs . as_bstr () , _ => false , } } }
    };
}

impl_71!();