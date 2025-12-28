macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! Entry32 {
    () => {
        deps!();
        # [doc = " A 32-byte completion queue entry (CQE), representing a complete I/O operation."] # [repr (C)] # [derive (Clone)] pub struct Entry32 (pub (crate) Entry , pub (crate) [u64 ; 2]) ;
    };
}

Entry32!();