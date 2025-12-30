// Generated macro for format_list (function)
macro_rules! Depcrate_query_builder_debug_queryformat_list {
() => {
// Module: crate::query_builder::debug_query
// Provides: {"format_list"}
// Dependencies: {}
fn format_list < 'b > (f : & mut fmt :: Formatter < '_ > , entries : & [Box < dyn Debug + 'b >]) -> fmt :: Result { let mut list = f . debug_list () ; for entry in entries { list . entry (entry) ; } list . finish () }
};
}
