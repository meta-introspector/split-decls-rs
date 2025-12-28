macro_rules! write_ncr {
    () => {
        # [doc = " Format an unmappable as NCR without heap allocation."] fn write_ncr (unmappable : char , dst : & mut [u8]) -> usize { let mut number = unmappable as u32 ; let len = if number >= 1_000_000u32 { 10usize } else if number >= 100_000u32 { 9usize } else if number >= 10_000u32 { 8usize } else if number >= 1_000u32 { 7usize } else if number >= 100u32 { 6usize } else { 5usize } ; debug_assert ! (number >= 10u32) ; debug_assert ! (len <= dst . len ()) ; let mut pos = len - 1 ; dst [pos] = b';' ; pos -= 1 ; loop { let rightmost = number % 10 ; dst [pos] = rightmost as u8 + b'0' ; pos -= 1 ; if number < 10 { break ; } number /= 10 ; } dst [1] = b'#' ; dst [0] = b'&' ; len }
    };
}

write_ncr!();