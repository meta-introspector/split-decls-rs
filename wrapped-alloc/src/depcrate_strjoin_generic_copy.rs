// Generated macro for join_generic_copy (function)
macro_rules! Depcrate_strjoin_generic_copy {
() => {
// Module: crate::str
// Provides: {"join_generic_copy"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] fn join_generic_copy < B , T , S > (slice : & [S] , sep : & [T]) -> Vec < T > where T : Copy , B : AsRef < [T] > + ? Sized , S : Borrow < B > , { let sep_len = sep . len () ; let mut iter = slice . iter () ; let first = match iter . next () { Some (first) => first , None => return vec ! [] , } ; let reserved_len = sep_len . checked_mul (iter . len ()) . and_then (| n | { slice . iter () . map (| s | s . borrow () . as_ref () . len ()) . try_fold (n , usize :: checked_add) }) . expect ("attempt to join into collection with len > usize::MAX") ; let mut result = Vec :: with_capacity (reserved_len) ; debug_assert ! (result . capacity () >= reserved_len) ; result . extend_from_slice (first . borrow () . as_ref ()) ; unsafe { let pos = result . len () ; let target = result . spare_capacity_mut () . get_unchecked_mut (.. reserved_len - pos) ; let sep_uninit = core :: slice :: from_raw_parts (sep . as_ptr () . cast () , sep . len ()) ; let iter_uninit = iter . map (| it | { let it = it . borrow () . as_ref () ; core :: slice :: from_raw_parts (it . as_ptr () . cast () , it . len ()) }) ; let remain = specialize_for_lengths ! (sep_uninit , target , iter_uninit ; 0 , 1 , 2 , 3 , 4) ; let result_len = reserved_len - remain . len () ; result . set_len (result_len) ; } result }
};
}
