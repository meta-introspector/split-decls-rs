// Generated macro for private (module)
macro_rules! Depcrateprivate {
() => {
// Module: crate
// Provides: {"private"}
// Dependencies: {}
mod private { # [doc (hidden)] pub trait Sealed : Copy { # [doc (hidden)] type Buffer : 'static ; fn write (self , buf : & mut Self :: Buffer) -> & str ; } }
};
}
