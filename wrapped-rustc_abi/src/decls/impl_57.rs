macro_rules! deps {
    () => {
        Endian!();
        AddressSpace!();
        PointerSpec!();
        TargetDataLayout!();
        Size!();
        Integer!();
        AbiAlign!();
        Align!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Default for TargetDataLayout { # [doc = " Creates an instance of `TargetDataLayout`."] fn default () -> TargetDataLayout { let align = | bits | Align :: from_bits (bits) . unwrap () ; TargetDataLayout { endian : Endian :: Big , i1_align : AbiAlign :: new (align (8)) , i8_align : AbiAlign :: new (align (8)) , i16_align : AbiAlign :: new (align (16)) , i32_align : AbiAlign :: new (align (32)) , i64_align : AbiAlign :: new (align (32)) , i128_align : AbiAlign :: new (align (32)) , f16_align : AbiAlign :: new (align (16)) , f32_align : AbiAlign :: new (align (32)) , f64_align : AbiAlign :: new (align (64)) , f128_align : AbiAlign :: new (align (128)) , aggregate_align : AbiAlign { abi : align (8) } , vector_align : vec ! [(Size :: from_bits (64) , AbiAlign :: new (align (64))) , (Size :: from_bits (128) , AbiAlign :: new (align (128))) ,] , default_address_space : AddressSpace :: ZERO , default_address_space_pointer_spec : PointerSpec { pointer_size : Size :: from_bits (64) , pointer_align : AbiAlign :: new (align (64)) , pointer_offset : Size :: from_bits (64) , _is_fat : false , } , address_space_info : vec ! [] , instruction_address_space : AddressSpace :: ZERO , c_enum_min_size : Integer :: I32 , } } }
    };
}

impl_57!();