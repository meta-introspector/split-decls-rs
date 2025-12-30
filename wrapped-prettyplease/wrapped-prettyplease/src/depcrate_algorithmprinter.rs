// Generated macro for Printer (struct)
macro_rules! Depcrate_algorithmPrinter {
() => {
// Module: crate::algorithm
// Provides: {"Printer"}
// Dependencies: {}
pub struct Printer { out : String , space : isize , buf : RingBuffer < BufEntry > , left_total : isize , right_total : isize , scan_stack : VecDeque < usize > , print_stack : Vec < PrintFrame > , indent : usize , pending_indentation : usize , }
};
}
