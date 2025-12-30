// Generated macro for take_fn (function)
macro_rules! Depcrate_parser_rangetake_fn {
() => {
// Module: crate::parser::range
// Provides: {"take_fn"}
// Dependencies: {}
# [doc = " Searches the entire range using `searcher` and then consumes a range of `Some(n)`."] # [doc = " If `f` can not find anything in the range it must return `None/NotFound` which indicates an end of input error."] # [doc = ""] # [doc = " If partial parsing is used the `TakeRange` enum can be returned instead of `Option`. By"] # [doc = " returning `TakeRange::NotFound(n)` it indicates that the input can skip ahead until `n`"] # [doc = " when parsing is next resumed."] # [doc = ""] # [doc = " See [`take_until_bytes`](../byte/fn.take_until_bytes.html) for a usecase."] pub fn take_fn < F , R , Input > (searcher : F) -> TakeFn < F , Input > where F : FnMut (Input :: Range) -> R , R : Into < TakeRange > , Input : RangeStream , Input :: Range : crate :: stream :: Range , { TakeFn { searcher , _marker : PhantomData , } }
};
}
