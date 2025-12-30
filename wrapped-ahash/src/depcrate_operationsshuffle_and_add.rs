// Generated macro for shuffle_and_add (function)
macro_rules! Depcrate_operationsshuffle_and_add {
() => {
// Module: crate::operations
// Provides: {"shuffle_and_add"}
// Dependencies: {}
# [allow (unused)] # [inline (always)] pub (crate) fn shuffle_and_add (base : u128 , to_add : u128) -> u128 { let shuffled : [u64 ; 2] = shuffle (base) . convert () ; add_by_64s (shuffled , to_add . convert ()) . convert () }
};
}
