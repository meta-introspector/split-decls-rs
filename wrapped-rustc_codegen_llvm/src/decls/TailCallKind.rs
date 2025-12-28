macro_rules! TailCallKind {
    () => {
        # [doc = " Must match the layout of `LLVMTailCallKind`."] # [derive (Copy , Clone , PartialEq , Debug)] # [repr (C)] # [allow (dead_code)] pub (crate) enum TailCallKind { None = 0 , Tail = 1 , MustTail = 2 , NoTail = 3 , }
    };
}

TailCallKind!()