// Generated macro for size_hint_and_one (function)
macro_rules! Depcrate_blinksize_hint_and_one {
() => {
// Module: crate::blink
// Provides: {"size_hint_and_one"}
// Dependencies: {}
# [inline] fn size_hint_and_one (lower : usize , upper : Option < usize > , count : usize) -> Option < usize > { let upper = upper . map_or (count , | upper | upper . min (count)) ; let size_hint = lower . max (upper) ; let size_hint = size_hint . checked_add (1) ? ; count . checked_add (size_hint) }
};
}
