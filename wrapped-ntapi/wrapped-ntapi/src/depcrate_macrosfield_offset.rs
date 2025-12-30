// Generated macro for FIELD_OFFSET (macro)
macro_rules! Depcrate_macrosFIELD_OFFSET {
() => {
// Module: crate::macros
// Provides: {"FIELD_OFFSET"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] macro_rules ! FIELD_OFFSET { ($ _type : ty , $ field : ident $ (.$ cfields : ident) *) => { { let obj = core :: mem :: MaybeUninit ::<$ _type >:: uninit () ; let base = obj . as_ptr () ; unsafe { core :: ptr :: addr_of ! ((* base) .$ field $ (.$ cfields) *) as usize - base as usize } } } ; }
};
}
