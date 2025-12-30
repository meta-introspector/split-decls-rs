// Generated macro for Drain (struct)
macro_rules! DepcrateDrain {
() => {
// Module: crate
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " An iterator over the exacted data by [`CompactString::drain()`]."] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct Drain < 'a > { compact_string : * mut CompactString , start : usize , end : usize , chars : core :: str :: Chars < 'a > , }
};
}
