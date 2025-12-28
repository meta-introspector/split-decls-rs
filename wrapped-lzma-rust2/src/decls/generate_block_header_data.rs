macro_rules! deps {
    () => {
        Result!();
        FilterConfig!();
        FilterType!();
    };
}

macro_rules! generate_block_header_data {
    () => {
        deps!();
        # [cfg (feature = "encoder")] fn generate_block_header_data (filters : & [FilterConfig] , lzma_dict_size : u32 ,) -> crate :: Result < Vec < u8 > > { let mut header_data = Vec :: new () ; let num_filters = filters . len () ; if num_filters > 4 { return Err (error_invalid_input ("too many filters in chain (maximum 4)")) ; } let block_flags = (num_filters - 1) as u8 ; header_data . push (block_flags) ; let mut temp_buf = [0u8 ; 10] ; for filter_config in filters { let filter_id = match filter_config . filter_type { FilterType :: Delta => 0x03 , FilterType :: BcjX86 => 0x04 , FilterType :: BcjPpc => 0x05 , FilterType :: BcjIa64 => 0x06 , FilterType :: BcjArm => 0x07 , FilterType :: BcjArmThumb => 0x08 , FilterType :: BcjSparc => 0x09 , FilterType :: BcjArm64 => 0x0A , FilterType :: BcjRiscv => 0x0B , FilterType :: Lzma2 => 0x21 , } ; let size = encode_multibyte_integer (filter_id , & mut temp_buf) ? ; header_data . extend_from_slice (& temp_buf [.. size]) ; match filter_config . filter_type { FilterType :: Delta => { let size = encode_multibyte_integer (1 , & mut temp_buf) ? ; header_data . extend_from_slice (& temp_buf [.. size]) ; let distance_prop = (filter_config . property - 1) as u8 ; header_data . push (distance_prop) ; } FilterType :: BcjX86 | FilterType :: BcjPpc | FilterType :: BcjIa64 | FilterType :: BcjArm | FilterType :: BcjArmThumb | FilterType :: BcjSparc | FilterType :: BcjArm64 | FilterType :: BcjRiscv => { if filter_config . property == 0 { let size = encode_multibyte_integer (0 , & mut temp_buf) ? ; header_data . extend_from_slice (& temp_buf [.. size]) ; } else { let size = encode_multibyte_integer (4 , & mut temp_buf) ? ; header_data . extend_from_slice (& temp_buf [.. size]) ; header_data . extend_from_slice (& filter_config . property . to_le_bytes ()) ; } } FilterType :: Lzma2 => { let size = encode_multibyte_integer (1 , & mut temp_buf) ? ; header_data . extend_from_slice (& temp_buf [.. size]) ; let dict_size_prop = encode_lzma2_dict_size (lzma_dict_size) ? ; header_data . push (dict_size_prop) ; } } } Ok (header_data) }
    };
}

generate_block_header_data!()