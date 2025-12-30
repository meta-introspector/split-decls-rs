// Generated macro for foreach_ordering (macro)
macro_rules! Depcrate_aarch64_linuxforeach_ordering {
() => {
// Module: crate::aarch64_linux
// Provides: {"foreach_ordering"}
// Dependencies: {}
# [macro_export] macro_rules ! foreach_ordering { ($ macro : path , $ bytes : tt , $ name : ident) => { $ macro ! (Relaxed , $ bytes , $ { concat ($ name , _relax) }) ; $ macro ! (Acquire , $ bytes , $ { concat ($ name , _acq) }) ; $ macro ! (Release , $ bytes , $ { concat ($ name , _rel) }) ; $ macro ! (AcqRel , $ bytes , $ { concat ($ name , _acq_rel) }) ; } ; ($ macro : path , $ name : ident) => { $ macro ! (Relaxed , $ { concat ($ name , _relax) }) ; $ macro ! (Acquire , $ { concat ($ name , _acq) }) ; $ macro ! (Release , $ { concat ($ name , _rel) }) ; $ macro ! (AcqRel , $ { concat ($ name , _acq_rel) }) ; } ; }
};
}
