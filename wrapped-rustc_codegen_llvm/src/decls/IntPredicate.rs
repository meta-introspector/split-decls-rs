macro_rules! IntPredicate {
    () => {
        # [doc = " LLVMIntPredicate"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum IntPredicate { IntEQ = 32 , IntNE = 33 , IntUGT = 34 , IntUGE = 35 , IntULT = 36 , IntULE = 37 , IntSGT = 38 , IntSGE = 39 , IntSLT = 40 , IntSLE = 41 , }
    };
}

IntPredicate!();