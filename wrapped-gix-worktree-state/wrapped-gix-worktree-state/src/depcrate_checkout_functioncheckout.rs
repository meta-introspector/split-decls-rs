// Generated macro for checkout (function)
macro_rules! Depcrate_checkout_functioncheckout {
() => {
// Module: crate::checkout::function
// Provides: {"checkout"}
// Dependencies: {}
# [doc = " Checkout the entire `index` into `dir`, and resolve objects found in index entries with `objects` to write their content to their"] # [doc = " respective path in `dir`."] # [doc = " Use `files` to count each fully checked out file, and count the amount written `bytes`. If `should_interrupt` is `true`, the"] # [doc = " operation will abort."] # [doc = " `options` provide a lot of context on how to perform the operation."] # [doc = ""] # [doc = " ### Handling the return value"] # [doc = ""] # [doc = " Note that interruption still produce an `Ok(…)` value, so the caller should look at `should_interrupt` to communicate the outcome."] # [doc = ""] # [allow (clippy :: too_many_arguments)] pub fn checkout < Find > (index : & mut gix_index :: State , dir : impl Into < std :: path :: PathBuf > , objects : Find , files : & dyn gix_features :: progress :: Count , bytes : & dyn gix_features :: progress :: Count , should_interrupt : & AtomicBool , options : crate :: checkout :: Options ,) -> Result < crate :: checkout :: Outcome , crate :: checkout :: Error > where Find : gix_object :: Find + Send + Clone , { let paths = index . take_path_backing () ; let res = checkout_inner (index , & paths , dir , objects , files , bytes , should_interrupt , options) ; index . return_path_backing (paths) ; res }
};
}
