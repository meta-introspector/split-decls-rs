// Generated macro for PluckConst (trait)
macro_rules! DepcratePluckConst {
() => {
// Module: crate
// Provides: {"PluckConst"}
// Dependencies: {}
# [doc = " *(internal)* Extracts the constant part `PluckedPart` at position `Index` from the partial"] # [doc = " reference having this trait, leaving `Self::Remainder`."] # [doc = ""] # [doc = " Plucking a constant part still leaves the part in the remaining reference, but will change it"] # [doc = " from mutable to constant."] # [doc = ""] # [doc = " The `Index` type can be inferred."] pub unsafe trait PluckConst < 'a , PluckedPart , Index > : PartialRef < 'a > { # [doc = " The partial reference left after plucking."] type Remainder : PartialRef < 'a , Target = Self :: Target > ; }
};
}
