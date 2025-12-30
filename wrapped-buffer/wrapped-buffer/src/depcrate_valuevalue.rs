// Generated macro for Value (struct)
macro_rules! Depcrate_valueValue {
() => {
// Module: crate::value
// Provides: {"Value"}
// Dependencies: {}
# [doc = "\nAn immutable buffered value.\n\nThis type is more compact than `ValueBuf`.\n"] # [derive (Debug , Clone)] pub struct Value < 'sval > { parts : Buf < ValuePart < 'sval > , 1 > , _marker : PhantomData < & 'sval () > , }
};
}
