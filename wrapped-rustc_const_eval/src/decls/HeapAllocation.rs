macro_rules! HeapAllocation {
    () => {
        # [derive (Debug)] pub (crate) struct HeapAllocation ;
    };
}

HeapAllocation!();