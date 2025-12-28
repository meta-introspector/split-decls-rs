macro_rules! deps {
    () => {
        FinalizeContext!();
    };
}

macro_rules! FinalizeFn {
    () => {
        deps!();
        type FinalizeFn < S > = Box < dyn Send + Sync + Fn (& mut FinalizeContext < '_ , '_ , S >) -> Option < AttributeKind > > ;
    };
}

FinalizeFn!();