macro_rules! deps {
    () => {
        LineIndex!();
        WideEncoding!();
        LineCol!();
    };
}

macro_rules! test_every_chars {
    () => {
        deps!();
        # [test] fn test_every_chars () { let text : String = { let mut chars : Vec < char > = ((0 as char) .. char :: MAX) . collect () ; chars . extend ("\n" . repeat (chars . len () / 16) . chars ()) ; let seed = std :: hash :: Hasher :: finish (& std :: hash :: BuildHasher :: build_hasher (# [allow (clippy :: disallowed_types)] & std :: collections :: hash_map :: RandomState :: new () ,)) ; let mut rng = oorandom :: Rand32 :: new (seed) ; let mut rand_index = | i | rng . rand_range (0 .. i as u32) as usize ; let mut remaining = chars . len () - 1 ; while remaining > 0 { let index = rand_index (remaining) ; chars . swap (remaining , index) ; remaining -= 1 ; } chars . into_iter () . collect () } ; assert ! (text . contains ('💩')) ; let line_index = LineIndex :: new (& text) ; let mut lin_col = LineCol { line : 0 , col : 0 } ; let mut col_utf16 = 0 ; let mut col_utf32 = 0 ; for (offset , c) in text . char_indices () { let got_offset = line_index . offset (lin_col) . unwrap () ; assert_eq ! (usize :: from (got_offset) , offset) ; let got_lin_col = line_index . line_col (got_offset) ; assert_eq ! (got_lin_col , lin_col) ; for (enc , col) in [(WideEncoding :: Utf16 , col_utf16) , (WideEncoding :: Utf32 , col_utf32)] { let wide_lin_col = line_index . to_wide (enc , lin_col) . unwrap () ; let got_lin_col = line_index . to_utf8 (enc , wide_lin_col) . unwrap () ; assert_eq ! (got_lin_col , lin_col) ; assert_eq ! (wide_lin_col . col , col) } if c == '\n' { lin_col . line += 1 ; lin_col . col = 0 ; col_utf16 = 0 ; col_utf32 = 0 ; } else { lin_col . col += c . len_utf8 () as u32 ; col_utf16 += c . len_utf16 () as u32 ; col_utf32 += 1 ; } } }
    };
}

test_every_chars!()