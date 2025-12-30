// Generated macro for impl_591 (impl)
macro_rules! Depcrate_concurrency_vector_clockimpl_591 {
() => {
// Module: crate::concurrency::vector_clock
// Provides: {"impl_591"}
// Dependencies: {}
impl VTimestamp { pub const ZERO : VTimestamp = VTimestamp :: new (0 , NaReadType :: Read , DUMMY_SP) ; # [inline] const fn encode_time_and_read_type (time : u32 , read_type : NaReadType) -> u32 { let read_type_bit = match read_type { NaReadType :: Read => 0 , NaReadType :: Retag => 1 , } ; read_type_bit | time . checked_mul (2) . expect ("Vector clock overflow") } # [inline] const fn new (time : u32 , read_type : NaReadType , span : Span) -> Self { Self { time_and_read_type : Self :: encode_time_and_read_type (time , read_type) , span } } # [inline] fn time (& self) -> u32 { self . time_and_read_type . shr (1) } # [inline] fn set_time (& mut self , time : u32) { self . time_and_read_type = Self :: encode_time_and_read_type (time , self . read_type ()) ; } # [inline] pub (super) fn read_type (& self) -> NaReadType { if self . time_and_read_type & 1 == 0 { NaReadType :: Read } else { NaReadType :: Retag } } # [inline] pub (super) fn set_read_type (& mut self , read_type : NaReadType) { self . time_and_read_type = Self :: encode_time_and_read_type (self . time () , read_type) ; } # [inline] pub (super) fn span_data (& self) -> SpanData { self . span . data () } }
};
}
