// Generated macro for sum_fields (function)
macro_rules! Depcrate_max_sizesum_fields {
() => {
// Module: crate::max_size
// Provides: {"sum_fields"}
// Dependencies: {}
fn sum_fields (fields : & Fields) -> TokenStream { match fields { syn :: Fields :: Named (fields) => { let recurse = fields . named . iter () . map (| f | { let ty = & f . ty ; quote_spanned ! { f . span () => <# ty as :: postcard :: experimental :: max_size :: MaxSize >:: POSTCARD_MAX_SIZE } }) ; quote ! { 0 # (+ # recurse) * } } syn :: Fields :: Unnamed (fields) => { let recurse = fields . unnamed . iter () . map (| f | { let ty = & f . ty ; quote_spanned ! { f . span () => <# ty as :: postcard :: experimental :: max_size :: MaxSize >:: POSTCARD_MAX_SIZE } }) ; quote ! { 0 # (+ # recurse) * } } syn :: Fields :: Unit => quote ! (0) , } }
};
}
