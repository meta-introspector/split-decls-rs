macro_rules! deps {
    () => {
        Endian!();
        Size!();
        PointerSpec!();
        Integer!();
        AbiAlign!();
        AddressSpace!();
    };
}

macro_rules! TargetDataLayout {
    () => {
        deps!();
        # [doc = " Parsed [Data layout](https://llvm.org/docs/LangRef.html#data-layout)"] # [doc = " for a target, which contains everything needed to compute layouts."] # [derive (Debug , PartialEq , Eq)] pub struct TargetDataLayout { pub endian : Endian , pub i1_align : AbiAlign , pub i8_align : AbiAlign , pub i16_align : AbiAlign , pub i32_align : AbiAlign , pub i64_align : AbiAlign , pub i128_align : AbiAlign , pub f16_align : AbiAlign , pub f32_align : AbiAlign , pub f64_align : AbiAlign , pub f128_align : AbiAlign , pub aggregate_align : AbiAlign , # [doc = " Alignments for vector types."] pub vector_align : Vec < (Size , AbiAlign) > , pub default_address_space : AddressSpace , pub default_address_space_pointer_spec : PointerSpec , # [doc = " Address space information of all known address spaces."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This vector does not contain the [`PointerSpec`] relative to the default address space,"] # [doc = " which instead lives in [`Self::default_address_space_pointer_spec`]."] address_space_info : Vec < (AddressSpace , PointerSpec) > , pub instruction_address_space : AddressSpace , # [doc = " Minimum size of #[repr(C)] enums (default c_int::BITS, usually 32)"] # [doc = " Note: This isn't in LLVM's data layout string, it is `short_enum`"] # [doc = " so the only valid spec for LLVM is c_int::BITS or 8"] pub c_enum_min_size : Integer , }
    };
}

TargetDataLayout!()