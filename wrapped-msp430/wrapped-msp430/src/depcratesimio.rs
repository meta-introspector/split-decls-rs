// Generated macro for simio (module)
macro_rules! Depcratesimio {
() => {
// Module: crate
// Provides: {"simio"}
// Dependencies: {}
mod simio { use core :: { convert :: Infallible , fmt } ; pub struct Console ; fn write_str (s : & str) { let addr = 0x00FF_usize as * mut u8 ; for & b in s . as_bytes () { unsafe { addr . write_volatile (b) } } } impl ufmt :: uWrite for Console { type Error = Infallible ; fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { write_str (s) ; Ok (()) } } impl fmt :: Write for Console { fn write_str (& mut self , s : & str) -> fmt :: Result { write_str (s) ; Ok (()) } } }
};
}
