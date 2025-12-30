// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_trivial_assert () { expect ! ["5"] . assert_eq ("5") ; } # [test] fn test_format_patch () { let patch = format_patch (None , "hello\nworld\n") ; expect ! [[r##"
            [r#"
            hello
            world
            "#]"##]] . assert_eq (& patch) ; let patch = format_patch (None , r"hello\tworld") ; expect ! [[r##"[r#"hello\tworld"#]"##]] . assert_eq (& patch) ; let patch = format_patch (None , "{\"foo\": 42}") ; expect ! [[r##"[r#"{"foo": 42}"#]"##]] . assert_eq (& patch) ; let patch = format_patch (Some (0) , "hello\nworld\n") ; expect ! [[r##"
            [r#"
                hello
                world
            "#]"##]] . assert_eq (& patch) ; let patch = format_patch (Some (4) , "single line") ; expect ! [[r#""single line""#]] . assert_eq (& patch) ; } # [test] fn test_patchwork () { let mut patchwork = Patchwork :: new ("one two three" . to_string ()) ; patchwork . patch (4 .. 7 , "zwei") ; patchwork . patch (0 .. 3 , "один") ; patchwork . patch (8 .. 13 , "3") ; expect ! [[r#"
            Patchwork {
                text: "один zwei 3",
                indels: [
                    (
                        0..3,
                        8,
                    ),
                    (
                        4..7,
                        4,
                    ),
                    (
                        8..13,
                        1,
                    ),
                ],
            }
        "#]] . assert_debug_eq (& patchwork) ; } # [test] fn test_expect_file () { expect_file ! ["./lib.rs"] . assert_eq (include_str ! ("./lib.rs")) } # [test] fn smoke_test_indent () { fn check_indented (input : & str , mut expect : Expect) { expect . indent (true) ; expect . assert_eq (input) ; } fn check_not_indented (input : & str , mut expect : Expect) { expect . indent (false) ; expect . assert_eq (input) ; } check_indented ("\
line1
  line2
" , expect ! [[r#"
                line1
                  line2
            "#]] ,) ; check_not_indented ("\
line1
  line2
" , expect ! [[r#"
line1
  line2
"#]] ,) ; } # [test] fn test_locate () { macro_rules ! check_locate { ($ ([[$ s : literal]]) ,* $ (,) ?) => { $ ({ let lit = stringify ! ($ s) ; let with_trailer = format ! ("{} \t]]\n" , lit) ; assert_eq ! (locate_end (& with_trailer) , Some (lit . len ())) ; }) * } ; } check_locate ! ([[r#"{ arr: [[1, 2], [3, 4]], other: "foo" } "#]] , [["]]"]] , [["\"]]"]] , [[r#""]]"#]] ,) ; assert_eq ! (locate_end ("]]") , Some (0)) ; } # [test] fn test_find_str_lit_len () { macro_rules ! check_str_lit_len { ($ ($ s : literal) ,* $ (,) ?) => { $ ({ let lit = stringify ! ($ s) ; assert_eq ! (find_str_lit_len (lit) , Some (lit . len ())) ; }) * } } check_str_lit_len ! [r##"foa\""#"## , r##"

                asdf][]]""""#
            "## , "" , "\"" , "\"\"" , "#\"#\"#" ,] ; } }
};
}
