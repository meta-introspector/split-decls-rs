macro_rules! RealPredicate {
    () => {
        # [doc = " LLVMRealPredicate"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum RealPredicate { RealPredicateFalse = 0 , RealOEQ = 1 , RealOGT = 2 , RealOGE = 3 , RealOLT = 4 , RealOLE = 5 , RealONE = 6 , RealORD = 7 , RealUNO = 8 , RealUEQ = 9 , RealUGT = 10 , RealUGE = 11 , RealULT = 12 , RealULE = 13 , RealUNE = 14 , RealPredicateTrue = 15 , }
    };
}

RealPredicate!()