// Generated macro for find_vtable_behind_cast (function)
macro_rules! Depcrate_debuginfo_metadatafind_vtable_behind_cast {
() => {
// Module: crate::debuginfo::metadata
// Provides: {"find_vtable_behind_cast"}
// Dependencies: {}
# [doc = " Get the global variable for the vtable."] # [doc = ""] # [doc = " When using global variables, we may have created an addrspacecast to get a pointer to the"] # [doc = " default address space if global variables are created in a different address space."] # [doc = " For modifying the vtable, we need the real global variable. This function accepts either a"] # [doc = " global variable (which is simply returned), or an addrspacecast constant expression."] # [doc = " If the given value is an addrspacecast, the cast is removed and the global variable behind"] # [doc = " the cast is returned."] fn find_vtable_behind_cast < 'll > (vtable : & 'll Value) -> & 'll Value { unsafe { if let Some (c) = llvm :: LLVMIsAConstantExpr (vtable) { if llvm :: LLVMGetConstOpcode (c) == llvm :: Opcode :: AddrSpaceCast { return llvm :: LLVMGetOperand (c , 0) . unwrap () ; } } } vtable }
};
}
