// Generated macro for decorate_function (function)
macro_rules! Depcrate_writedecorate_function {
() => {
// Module: crate::write
// Provides: {"decorate_function"}
// Dependencies: {}
# [doc = " Writes `func` to `w` as text."] # [doc = " write_function_plain is passed as 'closure' to print instructions as text."] # [doc = " pretty_function_error is passed as 'closure' to add error decoration."] pub fn decorate_function < FW : FuncWriter > (func_w : & mut FW , w : & mut dyn Write , func : & Function ,) -> fmt :: Result { write ! (w , "function ") ? ; write_spec (w , func) ? ; writeln ! (w , " {{") ? ; let aliases = alias_map (func) ; let mut any = func_w . write_preamble (w , func) ? ; for block in & func . layout { if any { writeln ! (w) ? ; } decorate_block (func_w , w , func , & aliases , block) ? ; any = true ; } writeln ! (w , "}}") }
};
}
