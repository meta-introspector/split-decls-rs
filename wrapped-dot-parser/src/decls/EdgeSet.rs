macro_rules! deps {
    () => {
        Edge!();
    };
}

macro_rules! EdgeSet {
    () => {
        deps!();
        # [doc = " A set of `Edge`s."] # [derive (Debug , Clone)] pub struct EdgeSet < A > { # [doc = " `Edge`s of the set."] pub set : Vec < Edge < A > > , }
    };
}

EdgeSet!()