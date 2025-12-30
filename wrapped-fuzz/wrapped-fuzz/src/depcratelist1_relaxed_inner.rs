// Generated macro for list1_relaxed_inner (function)
macro_rules! Depcratelist1_relaxed_inner {
() => {
// Module: crate
// Provides: {"list1_relaxed_inner"}
// Dependencies: {}
# [doc = " Parses `1#element` minus the leading and trailing portions."] fn list1_relaxed_inner < 'i , O , F > (f : F) -> impl FnMut (& 'i str) -> nom :: IResult < & 'i str , Vec < O > > where F : nom :: Parser < & 'i str , O , nom :: error :: Error < & 'i str > > , { separated_list1 (pair (ows , many1_count (pair (char (',') , ows))) , f) }
};
}
