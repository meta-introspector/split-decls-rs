// Generated macro for impl_734 (impl)
macro_rules! Depcrate_frameimpl_734 {
() => {
// Module: crate::frame
// Provides: {"impl_734"}
// Dependencies: {}
impl Ack { pub fn encode < W : BufMut > (delay : u64 , ranges : & ArrayRangeSet , ecn : Option < & EcnCounts > , buf : & mut W ,) { let mut rest = ranges . iter () . rev () ; let first = rest . next () . unwrap () ; let largest = first . end - 1 ; let first_size = first . end - first . start ; buf . write (if ecn . is_some () { FrameType :: ACK_ECN } else { FrameType :: ACK }) ; buf . write_var (largest) ; buf . write_var (delay) ; buf . write_var (ranges . len () as u64 - 1) ; buf . write_var (first_size - 1) ; let mut prev = first . start ; for block in rest { let size = block . end - block . start ; buf . write_var (prev - block . end - 1) ; buf . write_var (size - 1) ; prev = block . start ; } if let Some (x) = ecn { x . encode (buf) } } pub fn iter (& self) -> AckIter < '_ > { self . into_iter () } }
};
}
