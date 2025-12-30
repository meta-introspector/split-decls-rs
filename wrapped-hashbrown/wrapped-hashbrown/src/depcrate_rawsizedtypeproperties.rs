// Generated macro for SizedTypeProperties (trait)
macro_rules! Depcrate_rawSizedTypeProperties {
() => {
// Module: crate::raw
// Provides: {"SizedTypeProperties"}
// Dependencies: {}
trait SizedTypeProperties : Sized { const IS_ZERO_SIZED : bool = mem :: size_of :: < Self > () == 0 ; const NEEDS_DROP : bool = mem :: needs_drop :: < Self > () ; }
};
}
