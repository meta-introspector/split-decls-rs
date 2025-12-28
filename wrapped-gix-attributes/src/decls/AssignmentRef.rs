macro_rules! deps {
    () => {
        NameRef!();
        StateRef!();
    };
}

macro_rules! AssignmentRef {
    () => {
        deps!();
        # [doc = " Holds validated attribute data as a reference"] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub struct AssignmentRef < 'a > { # [doc = " The name of the attribute."] pub name : NameRef < 'a > , # [doc = " The state of the attribute."] pub state : StateRef < 'a > , }
    };
}

AssignmentRef!()