// Generated macro for write_block_header (function)
macro_rules! Depcrate_writewrite_block_header {
() => {
// Module: crate::write
// Provides: {"write_block_header"}
// Dependencies: {}
# [doc = " Write out the basic block header, outdented:"] # [doc = ""] # [doc = "    block1:"] # [doc = "    block1(v1: i32):"] # [doc = "    block10(v4: f64, v5: i8):"] # [doc = ""] pub fn write_block_header (w : & mut dyn Write , func : & Function , block : Block , indent : usize ,) -> fmt :: Result { let cold = if func . layout . is_cold (block) { " cold" } else { "" } ; write ! (w , "{1:0$}{2}" , indent - 4 , "" , block) ? ; let mut args = func . dfg . block_params (block) . iter () . cloned () ; match args . next () { None => return writeln ! (w , "{cold}:") , Some (arg) => { write ! (w , "(") ? ; write_arg (w , func , arg) ? ; } } for arg in args { write ! (w , ", ") ? ; write_arg (w , func , arg) ? ; } writeln ! (w , "){cold}:") }
};
}
