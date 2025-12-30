// Generated macro for PemLabel (trait)
macro_rules! DepcratePemLabel {
() => {
// Module: crate
// Provides: {"PemLabel"}
// Dependencies: {}
# [doc = " Marker trait for types with an associated PEM type label."] pub trait PemLabel { # [doc = " Expected PEM type label for a given document, e.g. `\"PRIVATE KEY\"`"] const PEM_LABEL : & 'static str ; # [doc = " Validate that a given label matches the expected label."] fn validate_pem_label (actual : & str) -> Result < () > { if Self :: PEM_LABEL == actual { Ok (()) } else { Err (Error :: UnexpectedTypeLabel { expected : Self :: PEM_LABEL , }) } } }
};
}
