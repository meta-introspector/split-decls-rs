// Generated macro for sealed (module)
macro_rules! Depcratesealed {
() => {
// Module: crate
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { pub trait Sealed { } impl < T : Clone > Sealed for T { } impl Sealed for str { } impl < T : Clone > Sealed for [T] { } pub struct Private ; }
};
}
