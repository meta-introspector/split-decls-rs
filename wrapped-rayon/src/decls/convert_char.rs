macro_rules! convert_char {
    () => {
        macro_rules ! convert_char { ($ self : ident . $ method : ident ($ ($ arg : expr) ,*)) => { if let Some ((start , end)) = $ self . bounds () { let start = start as u32 ; let end = end as u32 ; if start < 0xD800 && 0xE000 <= end { (start .. 0xD800) . into_par_iter () . chain (0xE000 .. end + 1) . map (| codepoint | unsafe { char :: from_u32_unchecked (codepoint) }) .$ method ($ ($ arg) ,*) } else { (start .. end + 1) . into_par_iter () . map (| codepoint | unsafe { char :: from_u32_unchecked (codepoint) }) .$ method ($ ($ arg) ,*) } } else { empty ::< char > () .$ method ($ ($ arg) ,*) } } ; }
    };
}

convert_char!()