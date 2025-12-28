macro_rules! deps {
    () => {
        NameRef!();
        AssignmentRef!();
        Assignment!();
        StateRef!();
    };
}

macro_rules! impl_0 {
    () => {
        deps!();
        impl < 'a > AssignmentRef < 'a > { pub (crate) fn new (name : NameRef < 'a > , state : StateRef < 'a >) -> AssignmentRef < 'a > { AssignmentRef { name , state } } # [doc = " Turn this reference into its owned counterpart."] pub fn to_owned (self) -> Assignment { self . into () } }
    };
}

impl_0!()