// Generated macro for dbg (macro)
macro_rules! Depcrate_macrosdbg {
() => {
// Module: crate::macros
// Provides: {"dbg"}
// Dependencies: {}
# [doc = " Prints and returns the value of a given expression for quick and dirty"] # [doc = " debugging."] # [cfg (target_os = "none")] # [macro_export] macro_rules ! dbg { () => { $ crate :: println ! ("[{}:{}]" , :: core :: file ! () , :: core :: line ! ()) } ; ($ val : expr $ (,) ?) => { match $ val { tmp => { $ crate :: println ! ("[{}:{}] {} = {:#?}" , :: core :: file ! () , :: core :: line ! () , :: core :: stringify ! ($ val) , & tmp) ; tmp } } } ; ($ ($ val : expr) ,+ $ (,) ?) => { ($ ($ crate :: dbg ! ($ val)) ,+,) } ; }
};
}
