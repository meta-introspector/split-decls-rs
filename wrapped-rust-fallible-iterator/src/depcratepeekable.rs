// Generated macro for Peekable (struct)
macro_rules! DepcratePeekable {
() => {
// Module: crate
// Provides: {"Peekable"}
// Dependencies: {}
# [doc = " An iterator which can look at the next element without consuming it."] # [derive (Clone , Debug)] pub struct Peekable < I : FallibleIterator > { it : I , next : Option < I :: Item > , }
};
}
