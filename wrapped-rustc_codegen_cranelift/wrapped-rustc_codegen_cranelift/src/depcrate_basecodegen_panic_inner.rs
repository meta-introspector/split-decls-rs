// Generated macro for codegen_panic_inner (function)
macro_rules! Depcrate_basecodegen_panic_inner {
() => {
// Module: crate::base
// Provides: {"codegen_panic_inner"}
// Dependencies: {}
fn codegen_panic_inner < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , lang_item : rustc_hir :: LangItem , args : & [Value] , _unwind : UnwindAction , span : Span ,) { fx . bcx . set_cold_block (fx . bcx . current_block () . unwrap ()) ; let def_id = fx . tcx . require_lang_item (lang_item , span) ; let instance = Instance :: mono (fx . tcx , def_id) ; if is_call_from_compiler_builtins_to_upstream_monomorphization (fx . tcx , instance) { fx . bcx . ins () . trap (TrapCode :: user (2) . unwrap ()) ; return ; } let symbol_name = fx . tcx . symbol_name (instance) . name ; fx . lib_call (symbol_name , args . iter () . map (| & arg | AbiParam :: new (fx . bcx . func . dfg . value_type (arg))) . collect () , vec ! [] , args ,) ; fx . bcx . ins () . trap (TrapCode :: user (1) . unwrap ()) ; }
};
}
