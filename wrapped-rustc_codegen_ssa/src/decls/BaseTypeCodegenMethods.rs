macro_rules! deps {
    () => {
        TypeKind!();
        BackendTypes!();
    };
}

macro_rules! BaseTypeCodegenMethods {
    () => {
        deps!();
        pub trait BaseTypeCodegenMethods : BackendTypes { fn type_i8 (& self) -> Self :: Type ; fn type_i16 (& self) -> Self :: Type ; fn type_i32 (& self) -> Self :: Type ; fn type_i64 (& self) -> Self :: Type ; fn type_i128 (& self) -> Self :: Type ; fn type_isize (& self) -> Self :: Type ; fn type_f16 (& self) -> Self :: Type ; fn type_f32 (& self) -> Self :: Type ; fn type_f64 (& self) -> Self :: Type ; fn type_f128 (& self) -> Self :: Type ; fn type_array (& self , ty : Self :: Type , len : u64) -> Self :: Type ; fn type_func (& self , args : & [Self :: Type] , ret : Self :: Type) -> Self :: Type ; fn type_kind (& self , ty : Self :: Type) -> TypeKind ; fn type_ptr (& self) -> Self :: Type ; fn type_ptr_ext (& self , address_space : AddressSpace) -> Self :: Type ; fn element_type (& self , ty : Self :: Type) -> Self :: Type ; # [doc = " Returns the number of elements in `self` if it is an LLVM vector type."] fn vector_length (& self , ty : Self :: Type) -> usize ; fn float_width (& self , ty : Self :: Type) -> usize ; # [doc = " Retrieves the bit width of the integer type `self`."] fn int_width (& self , ty : Self :: Type) -> u64 ; fn val_ty (& self , v : Self :: Value) -> Self :: Type ; }
    };
}

BaseTypeCodegenMethods!()