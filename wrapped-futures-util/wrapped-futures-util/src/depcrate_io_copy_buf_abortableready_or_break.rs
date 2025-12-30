// Generated macro for ready_or_break (macro)
macro_rules! Depcrate_io_copy_buf_abortableready_or_break {
() => {
// Module: crate::io::copy_buf_abortable
// Provides: {"ready_or_break"}
// Dependencies: {}
macro_rules ! ready_or_break { ($ e : expr $ (,) ?) => { match $ e { $ crate :: task :: Poll :: Ready (t) => t , $ crate :: task :: Poll :: Pending => break , } } ; }
};
}
