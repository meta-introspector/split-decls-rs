// Generated macro for collect_with_consumer (function)
macro_rules! Depcrate_iter_collectcollect_with_consumer {
() => {
// Module: crate::iter::collect
// Provides: {"collect_with_consumer"}
// Dependencies: {}
# [doc = " Create a consumer on the slice of memory we are collecting into."] # [doc = ""] # [doc = " The consumer needs to be used inside the scope function, and the"] # [doc = " complete collect result passed back."] # [doc = ""] # [doc = " This method will verify the collect result, and panic if the slice"] # [doc = " was not fully written into. Otherwise, in the successful case,"] # [doc = " the vector is complete with the collected result."] fn collect_with_consumer < T , F > (vec : & mut Vec < T > , len : usize , scope_fn : F) where T : Send , F : FnOnce (CollectConsumer < '_ , T >) -> CollectResult < '_ , T > , { vec . reserve (len) ; let result = scope_fn (CollectConsumer :: appender (vec , len)) ; let actual_writes = result . len () ; assert ! (actual_writes == len , "expected {len} total writes, but got {actual_writes}") ; result . release_ownership () ; let new_len = vec . len () + len ; unsafe { vec . set_len (new_len) ; } }
};
}
