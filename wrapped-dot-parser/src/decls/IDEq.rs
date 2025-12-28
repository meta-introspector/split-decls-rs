macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! IDEq {
    () => {
        deps!();
        # [derive (Clone , Debug)] # [doc = " An identifier equality, i.e. a statement that has the form `ID '=' ID`."] pub struct IDEq { # [doc = " The left hand side ID,"] pub lhs : String , # [doc = " The right hand side ID,"] pub rhs : String , }
    };
}

IDEq!();