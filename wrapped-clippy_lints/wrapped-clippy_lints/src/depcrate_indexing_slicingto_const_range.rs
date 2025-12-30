// Generated macro for to_const_range (function)
macro_rules! Depcrate_indexing_slicingto_const_range {
() => {
// Module: crate::indexing_slicing
// Provides: {"to_const_range"}
// Dependencies: {}
# [doc = " Returns a tuple of options with the start and end (exclusive) values of"] # [doc = " the range. If the start or end is not constant, None is returned."] fn to_const_range (cx : & LateContext < '_ > , range : higher :: Range < '_ > , array_size : u128) -> (Option < u128 > , Option < u128 >) { let ecx = ConstEvalCtxt :: new (cx) ; let s = range . start . map (| expr | ecx . eval (expr)) ; let start = match s { Some (Some (Constant :: Int (x))) => Some (x) , Some (_) => None , None => Some (0) , } ; let e = range . end . map (| expr | ecx . eval (expr)) ; let end = match e { Some (Some (Constant :: Int (x))) => { if range . limits == RangeLimits :: Closed { Some (x + 1) } else { Some (x) } } , Some (_) => None , None => Some (array_size) , } ; (start , end) }
};
}
