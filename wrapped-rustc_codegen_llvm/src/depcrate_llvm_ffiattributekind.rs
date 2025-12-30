// Generated macro for AttributeKind (enum)
macro_rules! Depcrate_llvm_ffiAttributeKind {
() => {
// Module: crate::llvm::ffi
// Provides: {"AttributeKind"}
// Dependencies: {}
# [doc = " Must match the layout of `LLVMRustAttributeKind`."] # [doc = " Semantically a subset of the C++ enum llvm::Attribute::AttrKind,"] # [doc = " though it is not ABI compatible (since it's a C++ enum)"] # [repr (C)] # [derive (Copy , Clone , Debug)] # [expect (dead_code , reason = "Some variants are unused, but are kept to match the C++")] pub (crate) enum AttributeKind { AlwaysInline = 0 , ByVal = 1 , Cold = 2 , InlineHint = 3 , MinSize = 4 , Naked = 5 , NoAlias = 6 , CapturesAddress = 7 , NoInline = 8 , NonNull = 9 , NoRedZone = 10 , NoReturn = 11 , NoUnwind = 12 , OptimizeForSize = 13 , ReadOnly = 14 , SExt = 15 , StructRet = 16 , UWTable = 17 , ZExt = 18 , InReg = 19 , SanitizeThread = 20 , SanitizeAddress = 21 , SanitizeMemory = 22 , NonLazyBind = 23 , OptimizeNone = 24 , ReadNone = 26 , SanitizeHWAddress = 28 , WillReturn = 29 , StackProtectReq = 30 , StackProtectStrong = 31 , StackProtect = 32 , NoUndef = 33 , SanitizeMemTag = 34 , NoCfCheck = 35 , ShadowCallStack = 36 , AllocSize = 37 , AllocatedPointer = 38 , AllocAlign = 39 , SanitizeSafeStack = 40 , FnRetThunkExtern = 41 , Writable = 42 , DeadOnUnwind = 43 , DeadOnReturn = 44 , CapturesReadOnly = 45 , }
};
}
