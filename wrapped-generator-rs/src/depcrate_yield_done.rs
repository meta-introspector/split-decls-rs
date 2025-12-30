// Generated macro for done (function)
macro_rules! Depcrate_yield_done {
() => {
// Module: crate::yield_
// Provides: {"done"}
// Dependencies: {}
# [doc = " don't use it directly, use done!() macro instead"] # [doc = " would panic if use in none generator context"] # [doc (hidden)] # [inline] pub fn done < T > () -> T { assert ! (is_generator () , "done is only possible in a generator") ; std :: panic :: panic_any (Error :: Done) }
};
}
