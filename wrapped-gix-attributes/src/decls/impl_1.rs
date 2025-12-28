macro_rules! deps {
    () => {
        AssignmentRef!();
        Assignment!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'a > From < AssignmentRef < 'a > > for Assignment { fn from (a : AssignmentRef < 'a >) -> Self { Assignment { name : a . name . to_owned () , state : a . state . to_owned () , } } }
    };
}

impl_1!()