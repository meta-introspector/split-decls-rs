macro_rules! deps {
    () => {
        Operator!();
    };
}

macro_rules! Assoc {
    () => {
        deps!();
        # [doc = " Associativity of an [`Operator`]."] # [doc = ""] # [doc = " [`Operator`]: struct.Operator.html"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum Assoc { # [doc = " Left `Operator` associativity"] Left , # [doc = " Right `Operator` associativity"] Right , }
    };
}

Assoc!()