macro_rules! deps {
    () => {
        Range!();
        Section!();
    };
}

macro_rules! macro_91 {
    () => {
        deps!();
        dw ! (# [doc = " Range list entry encoding values."] # [doc = ""] # [doc = " See Section 7.25, Table 7.30."] DwRle (u8) { DW_RLE_end_of_list = 0x00 , DW_RLE_base_addressx = 0x01 , DW_RLE_startx_endx = 0x02 , DW_RLE_startx_length = 0x03 , DW_RLE_offset_pair = 0x04 , DW_RLE_base_address = 0x05 , DW_RLE_start_end = 0x06 , DW_RLE_start_length = 0x07 , }) ;
    };
}

macro_91!()