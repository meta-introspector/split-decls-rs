macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! llvm_asm_scalar_type {
    () => {
        deps!();
        # [doc = " Helper function to get the LLVM type for a Scalar. Pointers are returned as"] # [doc = " the equivalent integer type."] fn llvm_asm_scalar_type < 'll > (cx : & CodegenCx < 'll , '_ > , scalar : Scalar) -> & 'll Type { let dl = & cx . tcx . data_layout ; match scalar . primitive () { Primitive :: Int (Integer :: I8 , _) => cx . type_i8 () , Primitive :: Int (Integer :: I16 , _) => cx . type_i16 () , Primitive :: Int (Integer :: I32 , _) => cx . type_i32 () , Primitive :: Int (Integer :: I64 , _) => cx . type_i64 () , Primitive :: Float (Float :: F16) => cx . type_f16 () , Primitive :: Float (Float :: F32) => cx . type_f32 () , Primitive :: Float (Float :: F64) => cx . type_f64 () , Primitive :: Float (Float :: F128) => cx . type_f128 () , Primitive :: Pointer (_) => cx . type_from_integer (dl . ptr_sized_integer ()) , _ => unreachable ! () , } }
    };
}

llvm_asm_scalar_type!();