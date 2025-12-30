// Generated macro for impl_259 (impl)
macro_rules! Depcrate_ioimpl_259 {
() => {
// Module: crate::io
// Provides: {"impl_259"}
// Dependencies: {}
impl < R : AsyncRead > BufReader < R > { # [doc = " Creates a buffered reader with the default buffer capacity."] # [doc = ""] # [doc = " The default capacity is currently 8 KB, but that may change in the future."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::BufReader;"] # [doc = ""] # [doc = " let input: &[u8] = b\"hello\";"] # [doc = " let reader = BufReader::new(input);"] # [doc = " ```"] pub fn new (inner : R) -> BufReader < R > { BufReader :: with_capacity (DEFAULT_BUF_SIZE , inner) } # [doc = " Creates a buffered reader with the specified capacity."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::BufReader;"] # [doc = ""] # [doc = " let input: &[u8] = b\"hello\";"] # [doc = " let reader = BufReader::with_capacity(1024, input);"] # [doc = " ```"] pub fn with_capacity (capacity : usize , inner : R) -> BufReader < R > { BufReader { inner , buf : vec ! [0 ; capacity] . into_boxed_slice () , pos : 0 , cap : 0 , } } }
};
}
