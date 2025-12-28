macro_rules! deps {
    () => {
        Assignment!();
        AssignmentRef!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'a > Assignment { # [doc = " Provide a ref type to this owned instance."] pub fn as_ref (& 'a self) -> AssignmentRef < 'a > { AssignmentRef :: new (self . name . as_ref () , self . state . as_ref ()) } }
    };
}

impl_2!();