// Generated macro for Printer (struct)
macro_rules! Depcrate_ppPrinter {
() => {
// Module: crate::pp
// Provides: {"Printer"}
// Dependencies: {}
pub struct Printer { out : String , # [doc = " Number of spaces left on line"] space : isize , # [doc = " Ring-buffer of tokens and calculated sizes"] buf : RingBuffer < BufEntry > , # [doc = " Running size of stream \"...left\""] left_total : isize , # [doc = " Running size of stream \"...right\""] right_total : isize , # [doc = " Pseudo-stack, really a ring too. Holds the"] # [doc = " primary-ring-buffers index of the Begin that started the"] # [doc = " current block, possibly with the most recent Break after that"] # [doc = " Begin (if there is any) on top of it. Stuff is flushed off the"] # [doc = " bottom as it becomes irrelevant due to the primary ring-buffer"] # [doc = " advancing."] scan_stack : VecDeque < usize > , # [doc = " Stack of blocks-in-progress being flushed by print"] print_stack : Vec < PrintFrame > , # [doc = " Level of indentation of current line"] indent : usize , # [doc = " Buffered indentation to avoid writing trailing whitespace"] pending_indentation : isize , # [doc = " The token most recently popped from the left boundary of the"] # [doc = " ring-buffer for printing"] last_printed : Option < Token > , }
};
}
