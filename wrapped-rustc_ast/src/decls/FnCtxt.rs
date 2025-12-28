macro_rules! deps {
    () => {
        AssocCtxt!();
    };
}

macro_rules! FnCtxt {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq)] pub enum FnCtxt { Free , Foreign , Assoc (AssocCtxt) , }
    };
}

FnCtxt!()