// Generated macro for if_signed (macro)
macro_rules! Depcrateif_signed {
() => {
// Module: crate
// Provides: {"if_signed"}
// Dependencies: {}
# [doc = " Output the given tokens if the type is signed, otherwise output nothing."] macro_rules ! if_signed { (true $ ($ x : tt) *) => { $ ($ x) * } ; (false $ ($ x : tt) *) => { } ; }
};
}
