macro_rules! deps {
    () => {
        UnalignedU16Slice!();
        CopyAsciiResult!();
        Endian!();
    };
}

macro_rules! convert_unaligned_utf16_to_utf8 {
    () => {
        deps!();
        # [inline (always)] fn convert_unaligned_utf16_to_utf8 < E : Endian > (src : UnalignedU16Slice , dst : & mut [u8] ,) -> (usize , usize , bool) { if dst . len () < 4 { return (0 , 0 , false) ; } let mut src_pos = 0usize ; let mut dst_pos = 0usize ; let src_len = src . len () ; let dst_len_minus_three = dst . len () - 3 ; 'outer : loop { let mut non_ascii = match copy_unaligned_basic_latin_to_ascii :: < E > (src . tail (src_pos) , & mut dst [dst_pos ..] ,) { CopyAsciiResult :: GoOn ((unit , read_written)) => { src_pos += read_written ; dst_pos += read_written ; unit } CopyAsciiResult :: Stop (read_written) => { return (src_pos + read_written , dst_pos + read_written , false) ; } } ; if dst_pos >= dst_len_minus_three { break 'outer ; } src_pos += 1 ; 'inner : loop { let non_ascii_minus_surrogate_start = non_ascii . wrapping_sub (0xD800) ; if non_ascii_minus_surrogate_start > (0xDFFF - 0xD800) { if non_ascii < 0x800 { dst [dst_pos] = ((non_ascii >> 6) | 0xC0) as u8 ; dst_pos += 1 ; dst [dst_pos] = ((non_ascii & 0x3F) | 0x80) as u8 ; dst_pos += 1 ; } else { dst [dst_pos] = ((non_ascii >> 12) | 0xE0) as u8 ; dst_pos += 1 ; dst [dst_pos] = (((non_ascii & 0xFC0) >> 6) | 0x80) as u8 ; dst_pos += 1 ; dst [dst_pos] = ((non_ascii & 0x3F) | 0x80) as u8 ; dst_pos += 1 ; } } else if non_ascii_minus_surrogate_start <= (0xDBFF - 0xD800) { if src_pos < src_len { let second = swap_if_opposite_endian :: < E > (src . at (src_pos)) ; let second_minus_low_surrogate_start = second . wrapping_sub (0xDC00) ; if second_minus_low_surrogate_start <= (0xDFFF - 0xDC00) { src_pos += 1 ; let point = (u32 :: from (non_ascii) << 10) + u32 :: from (second) - (((0xD800u32 << 10) - 0x10000u32) + 0xDC00u32) ; dst [dst_pos] = ((point >> 18) | 0xF0u32) as u8 ; dst_pos += 1 ; dst [dst_pos] = (((point & 0x3F000u32) >> 12) | 0x80u32) as u8 ; dst_pos += 1 ; dst [dst_pos] = (((point & 0xFC0u32) >> 6) | 0x80u32) as u8 ; dst_pos += 1 ; dst [dst_pos] = ((point & 0x3Fu32) | 0x80u32) as u8 ; dst_pos += 1 ; } else { return (src_pos , dst_pos , true) ; } } else { return (src_pos , dst_pos , true) ; } } else { return (src_pos , dst_pos , true) ; } if dst_pos >= dst_len_minus_three || src_pos == src_len { break 'outer ; } let unit = swap_if_opposite_endian :: < E > (src . at (src_pos)) ; src_pos += 1 ; if unit > 0x7F { non_ascii = unit ; continue 'inner ; } dst [dst_pos] = unit as u8 ; dst_pos += 1 ; continue 'outer ; } } (src_pos , dst_pos , false) }
    };
}

convert_unaligned_utf16_to_utf8!()