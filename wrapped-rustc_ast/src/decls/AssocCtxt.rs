macro_rules! deps {
    () => {
        Trait!();
        Impl!();
    };
}

macro_rules! AssocCtxt {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq)] pub enum AssocCtxt { Trait , Impl { of_trait : bool } , }
    };
}

AssocCtxt!();