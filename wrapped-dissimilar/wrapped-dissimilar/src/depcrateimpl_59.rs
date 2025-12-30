// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl Debug for Diff < '_ , '_ > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let (name , range) = match * self { Diff :: Equal (range , _) => ("Equal" , range) , Diff :: Delete (range) => ("Delete" , range) , Diff :: Insert (range) => ("Insert" , range) , } ; formatter . write_str (name) ? ; formatter . write_str ("(\"") ? ; for ch in range . chars () { if ch == '\'' { formatter . write_char (ch) ? ; } else { Display :: fmt (& ch . escape_debug () , formatter) ? ; } } formatter . write_str ("\")") ? ; Ok (()) } }
};
}
