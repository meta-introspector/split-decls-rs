macro_rules! InvocationStrategy {
    () => {
        # [derive (Clone , Debug , Default , PartialEq , Eq)] pub enum InvocationStrategy { Once , # [default] PerWorkspace , }
    };
}

InvocationStrategy!()