macro_rules! OverloadedDeref {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct OverloadedDeref (pub Mutability) ;
    };
}

OverloadedDeref!()