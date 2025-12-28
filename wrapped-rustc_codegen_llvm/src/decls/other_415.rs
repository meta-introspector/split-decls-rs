macro_rules! deps {
    () => {
        MetadataKindId!();
        AttributeKind!();
        LLVMRustVerifierFailureAction!();
        Bool!();
        Builder!();
    };
}

macro_rules! other_415 {
    () => {
        deps!();
        # [link (name = "llvm-wrapper" , kind = "static")] unsafe extern "C" { pub (crate) safe fn LLVMRustHasMetadata (I : & Value , KindID : MetadataKindId) -> bool ; pub (crate) fn LLVMRustEraseInstUntilInclusive (BB : & BasicBlock , I : & Value) ; pub (crate) fn LLVMRustGetLastInstruction < 'a > (BB : & BasicBlock) -> Option < & 'a Value > ; pub (crate) fn LLVMRustDIGetInstMetadata (I : & Value) -> Option < & Metadata > ; pub (crate) fn LLVMRustEraseInstFromParent (V : & Value) ; pub (crate) fn LLVMRustGetTerminator < 'a > (B : & BasicBlock) -> & 'a Value ; pub (crate) fn LLVMRustVerifyFunction (V : & Value , action : LLVMRustVerifierFailureAction) -> Bool ; pub (crate) fn LLVMRustHasAttributeAtIndex (V : & Value , i : c_uint , Kind : AttributeKind) -> bool ; pub (crate) fn LLVMRustGetArrayNumElements (Ty : & Type) -> u64 ; pub (crate) fn LLVMRustHasFnAttribute (F : & Value , Name : * const c_char , NameLen : libc :: size_t ,) -> bool ; pub (crate) fn LLVMRustRemoveFnAttribute (F : & Value , Name : * const c_char , NameLen : libc :: size_t) ; pub (crate) fn LLVMGetFirstFunction (M : & Module) -> Option < & Value > ; pub (crate) fn LLVMGetNextFunction (Fn : & Value) -> Option < & Value > ; pub (crate) fn LLVMRustRemoveEnumAttributeAtIndex (Fn : & Value , index : c_uint , kind : AttributeKind ,) ; pub (crate) fn LLVMRustPositionBefore < 'a > (B : & 'a Builder < '_ > , I : & 'a Value) ; pub (crate) fn LLVMRustPositionAfter < 'a > (B : & 'a Builder < '_ > , I : & 'a Value) ; pub (crate) fn LLVMRustGetFunctionCall (F : & Value , name : * const c_char , NameLen : libc :: size_t ,) -> Option < & Value > ; }
    };
}

other_415!();