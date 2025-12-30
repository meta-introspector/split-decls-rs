// Generated macro for MIRI_DEFAULT_ARGS (const)
macro_rules! DepcrateMIRI_DEFAULT_ARGS {
() => {
// Module: crate
// Provides: {"MIRI_DEFAULT_ARGS"}
// Dependencies: {}
# [doc = " Insert rustc arguments at the beginning of the argument list that Miri wants to be"] # [doc = " set per default, for maximal validation power."] # [doc = " Also disable the MIR pass that inserts an alignment check on every pointer dereference. Miri"] # [doc = " does that too, and with a better error message."] pub const MIRI_DEFAULT_ARGS : & [& str] = & ["--cfg=miri" , "-Zalways-encode-mir" , "-Zextra-const-ub-checks" , "-Zmir-emit-retag" , "-Zmir-preserve-ub" , "-Zmir-opt-level=0" , "-Zmir-enable-passes=-CheckAlignment,-CheckNull,-CheckEnums" , "-Zdeduplicate-diagnostics=no" ,] ;
};
}
