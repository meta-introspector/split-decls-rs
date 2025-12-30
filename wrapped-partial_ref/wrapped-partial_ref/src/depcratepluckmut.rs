// Generated macro for PluckMut (trait)
macro_rules! DepcratePluckMut {
() => {
// Module: crate
// Provides: {"PluckMut"}
// Dependencies: {}
# [doc = " *(internal)* Extracts the mutable part `PluckedPart` at position `Index` from the partial"] # [doc = " reference having this trait, leaving `Self::Remainder`."] # [doc = ""] # [doc = " Plucking a mutable part removes it from the remaining reference."] # [doc = ""] # [doc = " The `Index` type can be inferred."] pub unsafe trait PluckMut < 'a , PluckedPart , Index > : PartialRef < 'a > { # [doc = " The partial reference left after plucking."] type Remainder : PartialRef < 'a , Target = Self :: Target > ; }
};
}
