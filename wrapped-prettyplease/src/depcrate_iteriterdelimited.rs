// Generated macro for IterDelimited (trait)
macro_rules! Depcrate_iterIterDelimited {
() => {
// Module: crate::iter
// Provides: {"IterDelimited"}
// Dependencies: {}
pub trait IterDelimited : Iterator + Sized { fn delimited (self) -> Delimited < Self > { Delimited { is_first : true , iter : self . peekable () , } } }
};
}
