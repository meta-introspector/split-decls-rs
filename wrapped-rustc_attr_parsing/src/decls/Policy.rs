macro_rules! Policy {
    () => {
        # [derive (Debug , Eq , PartialEq)] pub (crate) enum Policy { Allow (Target) , Warn (Target) , Error (Target) , }
    };
}

Policy!()