// Generated macro for private (module)
macro_rules! Depcrate_strprivate {
() => {
// Module: crate::str
// Provides: {"private"}
// Dependencies: {}
# [doc = " We hide the `Pattern` trait in a private module, as its API is not meant"] # [doc = " for general consumption.  If we could have privacy on trait items, then it"] # [doc = " would be nicer to have its basic existence and implementors public while"] # [doc = " keeping all of the methods private."] mod private { use crate :: iter :: plumbing :: Folder ; # [doc = " Pattern-matching trait for `ParallelString`, somewhat like a mix of"] # [doc = " `std::str::pattern::{Pattern, Searcher}`."] # [doc = ""] # [doc = " Implementing this trait is not permitted outside of `rayon`."] pub trait Pattern : Sized + Sync + Send { private_decl ! { } fn find_in (& self , haystack : & str) -> Option < usize > ; fn rfind_in (& self , haystack : & str) -> Option < usize > ; fn is_suffix_of (& self , haystack : & str) -> bool ; fn fold_splits < 'ch , F > (& self , haystack : & 'ch str , folder : F , skip_last : bool) -> F where F : Folder < & 'ch str > ; fn fold_inclusive_splits < 'ch , F > (& self , haystack : & 'ch str , folder : F) -> F where F : Folder < & 'ch str > ; fn fold_matches < 'ch , F > (& self , haystack : & 'ch str , folder : F) -> F where F : Folder < & 'ch str > ; fn fold_match_indices < 'ch , F > (& self , haystack : & 'ch str , folder : F , base : usize) -> F where F : Folder < (usize , & 'ch str) > ; } }
};
}
