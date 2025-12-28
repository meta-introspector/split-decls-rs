macro_rules! CounterKind {
    () => {
        # [doc = " Must match the layout of `LLVMRustCounterKind`."] # [derive (Copy , Clone , Debug)] # [repr (C)] pub (crate) enum CounterKind { Zero = 0 , CounterValueReference = 1 , Expression = 2 , }
    };
}

CounterKind!()