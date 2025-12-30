// Generated macro for impl_330 (impl)
macro_rules! Depcrate_connection_assemblerimpl_330 {
() => {
// Module: crate::connection::assembler
// Provides: {"impl_330"}
// Dependencies: {}
impl Buffer { # [doc = " Constructs a new fragmented Buffer"] fn new (offset : u64 , bytes : Bytes , allocation_size : usize) -> Self { Self { offset , bytes , allocation_size , defragmented : false , } } # [doc = " Constructs a new defragmented Buffer"] fn new_defragmented (offset : u64 , bytes : Bytes) -> Self { let allocation_size = bytes . len () ; Self { offset , bytes , allocation_size , defragmented : true , } } # [doc = " Discards data before `offset` and flags `self` as defragmented if it has good utilization"] fn try_mark_defragment (& mut self , offset : u64) { let duplicate = offset . saturating_sub (self . offset) as usize ; self . offset = self . offset . max (offset) ; if duplicate >= self . bytes . len () { self . bytes = Bytes :: new () ; self . defragmented = true ; self . allocation_size = 0 ; return ; } self . bytes . advance (duplicate) ; self . defragmented = self . defragmented || self . bytes . len () * 6 / 5 >= self . allocation_size ; if self . defragmented { self . allocation_size = self . bytes . len () ; } } }
};
}
