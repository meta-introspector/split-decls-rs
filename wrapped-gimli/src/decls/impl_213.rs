macro_rules! deps {
    () => {
        UnwindContextStorage!();
        StoreOnHeap!();
        Register!();
        ReaderOffset!();
        RegisterRule!();
        UnwindTableRow!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        # [cfg (feature = "read")] impl < T : ReaderOffset > UnwindContextStorage < T > for StoreOnHeap { type Rules = [(Register , RegisterRule < T >) ; MAX_RULES] ; type Stack = Box < [UnwindTableRow < T , Self > ; MAX_UNWIND_STACK_DEPTH] > ; }
    };
}

impl_213!();