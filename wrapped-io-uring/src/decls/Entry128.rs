macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! Entry128 {
    () => {
        deps!();
        # [doc = " A 128-byte submission queue entry (SQE), representing a request for an I/O operation."] # [doc = ""] # [doc = " These can be created via opcodes in [`opcode`](crate::opcode)."] # [repr (C)] # [derive (Clone)] pub struct Entry128 (pub (crate) Entry , pub (crate) [u8 ; 64]) ;
    };
}

Entry128!()