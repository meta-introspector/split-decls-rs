// Generated macro for indent (function)
macro_rules! Depcrate_serindent {
() => {
// Module: crate::ser
// Provides: {"indent"}
// Dependencies: {}
fn indent < W > (wr : & mut W , n : usize , s : & [u8]) -> io :: Result < () > where W : ? Sized + io :: Write , { for _ in 0 .. n { tri ! (wr . write_all (s)) ; } Ok (()) }
};
}
