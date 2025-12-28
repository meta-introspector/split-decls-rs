macro_rules! deps {
    () => {
        UnwindContextStorage!();
        RegisterRuleMap!();
        ReaderOffset!();
        CfaRule!();
        StoreOnHeap!();
    };
}

macro_rules! UnwindTableRow {
    () => {
        deps!();
        # [doc = " A row in the virtual unwind table that describes how to find the values of"] # [doc = " the registers in the *previous* frame for a range of PC addresses."] # [derive (PartialEq , Eq)] pub struct UnwindTableRow < T , S = StoreOnHeap > where T : ReaderOffset , S : UnwindContextStorage < T > , { start_address : u64 , end_address : u64 , saved_args_size : u64 , cfa : CfaRule < T > , registers : RegisterRuleMap < T , S > , }
    };
}

UnwindTableRow!();