// Generated macro for dbg (macro)
macro_rules! Depcrate_loggingdbg {
() => {
// Module: crate::logging
// Provides: {"dbg"}
// Dependencies: {}
# [doc = " A copy of std's `dbg!` macro that doesn't do pretty printing."] # [doc = ""] # [doc = " This is nice because we usually want more compact output in this crate."] # [doc = " Also, because we don't import std's prelude, we have to use `std::dbg!`."] # [doc = " This macro definition makes it available as `dbg!`."] # [cfg (feature = "std")] macro_rules ! dbg { () => { std :: eprintln ! ("[{}:{}:{}]" , $ crate :: file ! () , $ crate :: line ! () , $ crate :: column ! () ,) } ; ($ val : expr $ (,) ?) => { match $ val { tmp => { std :: eprintln ! ("[{}:{}:{}] {} = {:?}" , std :: file ! () , std :: line ! () , std :: column ! () , std :: stringify ! ($ val) , & tmp ,) ; tmp } } } ; ($ ($ val : expr) ,+ $ (,) ?) => { ($ (dbg ! ($ val)) ,+,) } ; }
};
}
