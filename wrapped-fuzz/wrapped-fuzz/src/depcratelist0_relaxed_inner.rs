// Generated macro for list0_relaxed_inner (function)
macro_rules! Depcratelist0_relaxed_inner {
() => {
// Module: crate
// Provides: {"list0_relaxed_inner"}
// Dependencies: {}
# [doc = " Parses `1#element` minus the leading and trailing portions."] # [doc = ""] # [doc = " This is used in the `challenge` definition; it avoids ambiguities with"] # [doc = " the outer list1."] fn list0_relaxed_inner < 'i , O , F > (f : F) -> impl FnMut (& 'i str) -> nom :: IResult < & 'i str , Vec < O > > where F : nom :: Parser < & 'i str , O , nom :: error :: Error < & 'i str > > , { separated_list0 (pair (ows , many1_count (pair (char (',') , ows))) , f) }
};
}
