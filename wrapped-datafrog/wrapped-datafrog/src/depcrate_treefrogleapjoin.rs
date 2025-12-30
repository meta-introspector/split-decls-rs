// Generated macro for leapjoin (function)
macro_rules! Depcrate_treefrogleapjoin {
() => {
// Module: crate::treefrog
// Provides: {"leapjoin"}
// Dependencies: {}
# [doc = " Performs treefrog leapjoin using a list of leapers."] pub (crate) fn leapjoin < 'leap , Tuple : Ord , Val : Ord + 'leap , Result : Ord > (source : & [Tuple] , mut leapers : impl Leapers < 'leap , Tuple , Val > , mut logic : impl FnMut (& Tuple , & Val) -> Result ,) -> Relation < Result > { let mut result = Vec :: new () ; let mut values = Vec :: new () ; for tuple in source { let mut min_index = usize :: max_value () ; let mut min_count = usize :: max_value () ; leapers . for_each_count (tuple , | index , count | { if min_count > count { min_count = count ; min_index = index ; } }) ; assert ! (min_count < usize :: max_value ()) ; if min_count > 0 { leapers . propose (tuple , min_index , & mut values) ; leapers . intersect (tuple , min_index , & mut values) ; for val in values . drain (..) { result . push (logic (tuple , val)) ; } } } Relation :: from_vec (result) }
};
}
