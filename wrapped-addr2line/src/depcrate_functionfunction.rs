// Generated macro for Function (struct)
macro_rules! Depcrate_functionFunction {
() => {
// Module: crate::function
// Provides: {"Function"}
// Dependencies: {}
pub (crate) struct Function < R : gimli :: Reader > { pub (crate) dw_die_offset : gimli :: UnitOffset < R :: Offset > , pub (crate) name : Option < R > , # [doc = " List of all `DW_TAG_inlined_subroutine` details in this function."] inlined_functions : Box < [InlinedFunction < R >] > , # [doc = " List of `DW_TAG_inlined_subroutine` address ranges in this function."] inlined_addresses : Box < [InlinedFunctionAddress] > , }
};
}
