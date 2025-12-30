// Generated macro for checkout_exclusive (module)
macro_rules! Depcrate_indexcheckout_exclusive {
() => {
// Module: crate::index
// Provides: {"checkout_exclusive"}
// Dependencies: {}
pub mod checkout_exclusive { pub struct Options { pub index : super :: Options , # [doc = " If true, all files will be written with zero bytes despite having made an ODB lookup."] pub empty_files : bool , pub keep_going : bool , # [doc = " If set, don't use more than this amount of threads."] # [doc = " Otherwise, usually use as many threads as there are logical cores."] # [doc = " A value of 0 is interpreted as no-limit"] pub thread_limit : Option < usize > , } }
};
}
