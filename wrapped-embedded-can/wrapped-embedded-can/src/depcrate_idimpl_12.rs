// Generated macro for impl_12 (impl)
macro_rules! Depcrate_idimpl_12 {
() => {
// Module: crate::id
// Provides: {"impl_12"}
// Dependencies: {}
# [doc = " Implement `Ord` according to the CAN arbitration rules"] # [doc = ""] # [doc = " When performing arbitration, frames are looked at bit for bit starting"] # [doc = " from the beginning. A bit with the value 0 is dominant and a bit with"] # [doc = " value of 1 is recessive."] # [doc = ""] # [doc = " When two devices are sending frames at the same time, as soon as the first"] # [doc = " bit is found which differs, the frame with the corresponding dominant"] # [doc = " 0 bit will win and get to send the rest of the frame."] # [doc = ""] # [doc = " This implementation of `Ord` for `Id` will take this into consideration"] # [doc = " and when comparing two different instances of `Id` the \"smallest\" will"] # [doc = " always be the ID which would form the most dominant frame, all other"] # [doc = " things being equal."] impl Ord for Id { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { let split_id = | id : & Id | { let (standard_id_part , ide_bit , extended_id_part) = match id { Id :: Standard (StandardId (x)) => (* x , 0 , 0) , Id :: Extended (x) => (x . standard_id () . 0 , 1 , x . 0 & ((1 << 18) - 1) ,) , } ; (standard_id_part , ide_bit , extended_id_part) } ; split_id (self) . cmp (& split_id (other)) } }
};
}
