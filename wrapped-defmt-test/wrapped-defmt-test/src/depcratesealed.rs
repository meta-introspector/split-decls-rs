// Generated macro for sealed (module)
macro_rules! Depcratesealed {
() => {
// Module: crate
// Provides: {"sealed"}
// Dependencies: {}
mod sealed { pub trait Sealed { } impl Sealed for () { } impl < T , E > Sealed for Result < T , E > { } }
};
}
