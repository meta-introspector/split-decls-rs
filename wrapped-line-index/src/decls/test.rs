macro_rules! deps {
    () => {
        LineIndex!();
        WideChar!();
    };
}

macro_rules! test {
    () => {
        deps!();
        macro_rules ! test { (case : $ test_name : ident , text : $ text : expr , lines : $ lines : expr , multi_byte_chars : $ multi_byte_chars : expr ,) => { # [test] fn $ test_name () { let line_index = LineIndex :: new ($ text) ; let expected_lines : Vec < TextSize > = $ lines . into_iter () . map (< TextSize as From < u32 >>:: from) . collect () ; assert_eq ! (&* line_index . newlines , &* expected_lines) ; let expected_mbcs : Vec < _ > = $ multi_byte_chars . into_iter () . map (| (line , (pos , end)) : (u32 , (u32 , u32)) | { (line , WideChar { start : TextSize :: from (pos) , end : TextSize :: from (end) }) }) . collect () ; assert_eq ! (line_index . line_wide_chars . iter () . flat_map (| (line , val) | std :: iter :: repeat (* line) . zip (val . iter () . copied ())) . collect ::< Vec < _ >> () , expected_mbcs) ; } } ; }
    };
}

test!()