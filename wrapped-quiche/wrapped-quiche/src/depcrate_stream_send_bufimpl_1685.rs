// Generated macro for impl_1685 (impl)
macro_rules! Depcrate_stream_send_bufimpl_1685 {
() => {
// Module: crate::stream::send_buf
// Provides: {"impl_1685"}
// Dependencies: {}
impl < F : BufFactory > SendReserve < '_ , F > { fn append_buf (& mut self , buf : F :: Buf) -> Result < () > { let len = buf . as_ref () . len () ; let inner = & mut self . inner ; if len > self . reserved { return Err (Error :: BufferTooShort) ; } let fin : bool = self . reserved == len && self . fin ; let buf = RangeBuf :: from_raw (buf , inner . off , fin) ; inner . data . push_back (buf) ; inner . off += len as u64 ; inner . len += len as u64 ; self . reserved -= len ; Ok (()) } }
};
}
