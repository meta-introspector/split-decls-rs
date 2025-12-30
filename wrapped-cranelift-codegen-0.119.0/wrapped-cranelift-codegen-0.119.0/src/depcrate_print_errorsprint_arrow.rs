// Generated macro for print_arrow (function)
macro_rules! Depcrate_print_errorsprint_arrow {
() => {
// Module: crate::print_errors
// Provides: {"print_arrow"}
// Dependencies: {}
# [doc = " Prints:"] # [doc = "    ;   ^~~~~~"] fn print_arrow (w : & mut dyn Write , entity : & str) -> fmt :: Result { write ! (w , ";") ? ; let indent = entity . len () - entity . trim_start () . len () ; if indent != 0 { write ! (w , "{1:0$}^" , indent - 1 , "") ? ; } for _ in 0 .. entity . trim () . len () - 1 { write ! (w , "~") ? ; } writeln ! (w) }
};
}
