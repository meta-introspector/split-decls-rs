macro_rules! TypeKind {
    () => {
        # [doc = " Must match the layout of `LLVMTypeKind`."] # [doc = ""] # [doc = " Use [`RawEnum<TypeKind>`] for values of `LLVMTypeKind` returned from LLVM,"] # [doc = " to avoid risk of UB if LLVM adds new enum values."] # [doc = ""] # [doc = " All of LLVM's variants should be declared here, even if no Rust-side code refers"] # [doc = " to them, because unknown variants will cause [`RawEnum::to_rust`] to panic."] # [derive (Copy , Clone , PartialEq , Debug , TryFromU32)] # [repr (C)] pub (crate) enum TypeKind { Void = 0 , Half = 1 , Float = 2 , Double = 3 , X86_FP80 = 4 , FP128 = 5 , PPC_FP128 = 6 , Label = 7 , Integer = 8 , Function = 9 , Struct = 10 , Array = 11 , Pointer = 12 , Vector = 13 , Metadata = 14 , Token = 16 , ScalableVector = 17 , BFloat = 18 , X86_AMX = 19 , }
    };
}

TypeKind!();