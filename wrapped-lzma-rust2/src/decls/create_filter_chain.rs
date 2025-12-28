macro_rules! deps {
    () => {
        Lzma2Reader!();
        Read!();
        FilterType!();
    };
}

macro_rules! create_filter_chain {
    () => {
        deps!();
        # [cfg (feature = "std")] fn create_filter_chain < 'reader > (mut chain_reader : Box < dyn Read + 'reader > , filters : & [Option < FilterType >] , properties : & [u32] ,) -> Box < dyn Read + 'reader > { for (filter , property) in filters . iter () . copied () . zip (properties) . filter_map (| (filter , property) | filter . map (| filter | (filter , * property))) . rev () { chain_reader = match filter { FilterType :: Delta => { let distance = property as usize ; Box :: new (DeltaReader :: new (chain_reader , distance)) } FilterType :: BcjX86 => { let start_offset = property as usize ; Box :: new (BcjReader :: new_x86 (chain_reader , start_offset)) } FilterType :: BcjPpc => { let start_offset = property as usize ; Box :: new (BcjReader :: new_ppc (chain_reader , start_offset)) } FilterType :: BcjIa64 => { let start_offset = property as usize ; Box :: new (BcjReader :: new_ia64 (chain_reader , start_offset)) } FilterType :: BcjArm => { let start_offset = property as usize ; Box :: new (BcjReader :: new_arm (chain_reader , start_offset)) } FilterType :: BcjArmThumb => { let start_offset = property as usize ; Box :: new (BcjReader :: new_arm_thumb (chain_reader , start_offset)) } FilterType :: BcjSparc => { let start_offset = property as usize ; Box :: new (BcjReader :: new_sparc (chain_reader , start_offset)) } FilterType :: BcjArm64 => { let start_offset = property as usize ; Box :: new (BcjReader :: new_arm64 (chain_reader , start_offset)) } FilterType :: BcjRiscv => { let start_offset = property as usize ; Box :: new (BcjReader :: new_riscv (chain_reader , start_offset)) } FilterType :: Lzma2 => { let dict_size = property ; Box :: new (Lzma2Reader :: new (chain_reader , dict_size , None)) } } ; } chain_reader }
    };
}

create_filter_chain!()