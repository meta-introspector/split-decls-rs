// Generated macro for impl_905 (impl)
macro_rules! Depcrate_util_path_listimpl_905 {
() => {
// Module: crate::util::path_list
// Provides: {"impl_905"}
// Dependencies: {}
impl PathList { # [doc = " Create a new list."] pub fn new < T : Into < Path > > (vals : Vec < T >) -> Self { PathList (vals . into_iter () . map (T :: into) . collect ()) } # [doc = " Create a new `Vec` containing the string representation of each path."] pub fn to_strings (& self) -> Vec < String > { self . 0 . iter () . map (path_to_string) . collect () } # [doc = " Visits the values representing the intersection, i.e., the values that are both in `self` and `other`."] # [doc = ""] # [doc = " Values will be returned in the order they appear in `self`."] # [doc = ""] # [doc = " Values that are present multiple times in `self` and present at least once in `other` will be returned multiple times."] # [doc = ""] # [doc = " # Performance"] # [doc = " This function runs in `O(n * m)` time."] # [doc = " It is believed that path lists are usually short enough that this is better than allocating a set containing the values"] # [doc = " of `other` or adding a conditional solution."] pub fn intersection < 'a > (& 'a self , other : & 'a PathList) -> impl Iterator < Item = & 'a Path > { self . 0 . iter () . filter (| path | other . 0 . contains (path)) } }
};
}
