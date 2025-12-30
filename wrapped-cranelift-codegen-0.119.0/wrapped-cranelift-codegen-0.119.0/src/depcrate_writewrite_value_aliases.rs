// Generated macro for write_value_aliases (function)
macro_rules! Depcrate_writewrite_value_aliases {
() => {
// Module: crate::write
// Provides: {"write_value_aliases"}
// Dependencies: {}
# [doc = " Write out any aliases to the given target, including indirect aliases"] fn write_value_aliases (w : & mut dyn Write , aliases : & SecondaryMap < Value , Vec < Value > > , target : Value , indent : usize ,) -> fmt :: Result { let mut todo_stack = vec ! [target] ; while let Some (target) = todo_stack . pop () { for & a in & aliases [target] { writeln ! (w , "{1:0$}{2} -> {3}" , indent , "" , a , target) ? ; todo_stack . push (a) ; } } Ok (()) }
};
}
