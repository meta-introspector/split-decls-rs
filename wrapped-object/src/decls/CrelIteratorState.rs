macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! CrelIteratorState {
    () => {
        deps!();
        # [derive (Default , Debug , Clone)] struct CrelIteratorState { # [doc = " Index of the current relocation."] index : usize , # [doc = " Offset of the latest relocation."] offset : u64 , # [doc = " Addend of the latest relocation."] addend : i64 , # [doc = " Symbol index of the latest relocation."] symidx : u32 , # [doc = " Type of the latest relocation."] typ : u32 , }
    };
}

CrelIteratorState!();