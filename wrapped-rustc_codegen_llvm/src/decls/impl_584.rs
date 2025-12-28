macro_rules! deps {
    () => {
        GenericCx!();
        SCx!();
        TypeKind!();
    };
}

macro_rules! impl_584 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > BaseTypeCodegenMethods for GenericCx < 'll , CX > { fn type_i8 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt8TypeInContext (self . llcx ()) } } fn type_i16 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt16TypeInContext (self . llcx ()) } } fn type_i32 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt32TypeInContext (self . llcx ()) } } fn type_i64 (& self) -> & 'll Type { unsafe { llvm :: LLVMInt64TypeInContext (self . llcx ()) } } fn type_i128 (& self) -> & 'll Type { unsafe { llvm :: LLVMIntTypeInContext (self . llcx () , 128) } } fn type_isize (& self) -> & 'll Type { self . isize_ty () } fn type_f16 (& self) -> & 'll Type { unsafe { llvm :: LLVMHalfTypeInContext (self . llcx ()) } } fn type_f32 (& self) -> & 'll Type { unsafe { llvm :: LLVMFloatTypeInContext (self . llcx ()) } } fn type_f64 (& self) -> & 'll Type { unsafe { llvm :: LLVMDoubleTypeInContext (self . llcx ()) } } fn type_f128 (& self) -> & 'll Type { unsafe { llvm :: LLVMFP128TypeInContext (self . llcx ()) } } fn type_func (& self , args : & [& 'll Type] , ret : & 'll Type) -> & 'll Type { unsafe { llvm :: LLVMFunctionType (ret , args . as_ptr () , args . len () as c_uint , FALSE) } } fn type_kind (& self , ty : & 'll Type) -> TypeKind { llvm :: LLVMGetTypeKind (ty) . to_rust () . to_generic () } fn type_ptr (& self) -> & 'll Type { self . type_ptr_ext (AddressSpace :: ZERO) } fn type_ptr_ext (& self , address_space : AddressSpace) -> & 'll Type { unsafe { llvm :: LLVMPointerTypeInContext (self . llcx () , address_space . 0) } } fn element_type (& self , ty : & 'll Type) -> & 'll Type { match self . type_kind (ty) { TypeKind :: Array | TypeKind :: Vector => unsafe { llvm :: LLVMGetElementType (ty) } , TypeKind :: Pointer => bug ! ("element_type is not supported for opaque pointers") , other => bug ! ("element_type called on unsupported type {other:?}") , } } fn vector_length (& self , ty : & 'll Type) -> usize { unsafe { llvm :: LLVMGetVectorSize (ty) as usize } } fn float_width (& self , ty : & 'll Type) -> usize { match self . type_kind (ty) { TypeKind :: Half => 16 , TypeKind :: Float => 32 , TypeKind :: Double => 64 , TypeKind :: X86_FP80 => 80 , TypeKind :: FP128 | TypeKind :: PPC_FP128 => 128 , other => bug ! ("llvm_float_width called on a non-float type {other:?}") , } } fn int_width (& self , ty : & 'll Type) -> u64 { unsafe { llvm :: LLVMGetIntTypeWidth (ty) as u64 } } fn val_ty (& self , v : & 'll Value) -> & 'll Type { common :: val_ty (v) } fn type_array (& self , ty : & 'll Type , len : u64) -> & 'll Type { unsafe { llvm :: LLVMArrayType2 (ty , len) } } }
    };
}

impl_584!();