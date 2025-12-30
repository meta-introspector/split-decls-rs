// Generated macro for sealed (module)
macro_rules! Depcrate_typessealed {
() => {
// Module: crate::types
// Provides: {"sealed"}
// Dependencies: {}
pub (crate) mod sealed { use super :: { Fd , Fixed } ; use std :: os :: unix :: io :: RawFd ; # [derive (Debug)] pub enum Target { Fd (RawFd) , Fixed (u32) , } pub trait UseFd : Sized { fn into (self) -> RawFd ; } pub trait UseFixed : Sized { fn into (self) -> Target ; } impl UseFd for Fd { # [inline] fn into (self) -> RawFd { self . 0 } } impl UseFixed for Fd { # [inline] fn into (self) -> Target { Target :: Fd (self . 0) } } impl UseFixed for Fixed { # [inline] fn into (self) -> Target { Target :: Fixed (self . 0) } } }
};
}
