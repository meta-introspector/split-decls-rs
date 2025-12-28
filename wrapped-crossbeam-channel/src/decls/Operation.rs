macro_rules! Operation {
    () => {
        # [doc = " Identifier associated with an operation by a specific thread on a specific channel."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Operation (usize) ;
    };
}

Operation!()