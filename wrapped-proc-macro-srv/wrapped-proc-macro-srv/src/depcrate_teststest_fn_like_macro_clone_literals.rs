// Generated macro for test_fn_like_macro_clone_literals (function)
macro_rules! Depcrate_teststest_fn_like_macro_clone_literals {
() => {
// Module: crate::tests
// Provides: {"test_fn_like_macro_clone_literals"}
// Dependencies: {}
# [test] fn test_fn_like_macro_clone_literals () { assert_expand ("fn_like_clone_tokens" , r###"1u16, 2_u32, -4i64, 3.14f32, "hello bridge", "suffixed"suffix, r##"raw"##, 'a', b'b', c"null""### , expect ! [[r#"
            SUBTREE $$ 1 1
              LITERAL Integer 1u16 1
              PUNCH   , [alone] 1
              LITERAL Integer 2_u32 1
              PUNCH   , [alone] 1
              PUNCH   - [alone] 1
              LITERAL Integer 4i64 1
              PUNCH   , [alone] 1
              LITERAL Float 3.14f32 1
              PUNCH   , [alone] 1
              LITERAL Str hello bridge 1
              PUNCH   , [alone] 1
              LITERAL Err(()) "suffixed"suffix 1
              PUNCH   , [alone] 1
              LITERAL StrRaw(2) raw 1
              PUNCH   , [alone] 1
              LITERAL Char a 1
              PUNCH   , [alone] 1
              LITERAL Byte b 1
              PUNCH   , [alone] 1
              LITERAL CStr null 1



            SUBTREE $$ 1 1
              LITERAL Integer 1u16 1
              PUNCH   , [alone] 1
              LITERAL Integer 2_u32 1
              PUNCH   , [alone] 1
              PUNCH   - [alone] 1
              LITERAL Integer 4i64 1
              PUNCH   , [alone] 1
              LITERAL Float 3.14f32 1
              PUNCH   , [alone] 1
              LITERAL Str hello bridge 1
              PUNCH   , [alone] 1
              LITERAL Str suffixedsuffix 1
              PUNCH   , [alone] 1
              LITERAL StrRaw(2) raw 1
              PUNCH   , [alone] 1
              LITERAL Char a 1
              PUNCH   , [alone] 1
              LITERAL Byte b 1
              PUNCH   , [alone] 1
              LITERAL CStr null 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Integer 1u16 42:Root[0000, 0]@0..4#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@4..5#ROOT2024
              LITERAL Integer 2_u32 42:Root[0000, 0]@6..11#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@11..12#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@13..14#ROOT2024
              LITERAL Integer 4i64 42:Root[0000, 0]@14..18#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@18..19#ROOT2024
              LITERAL Float 3.14f32 42:Root[0000, 0]@20..27#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@27..28#ROOT2024
              LITERAL Str hello bridge 42:Root[0000, 0]@29..43#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@43..44#ROOT2024
              LITERAL Err(()) "suffixed"suffix 42:Root[0000, 0]@45..61#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@61..62#ROOT2024
              LITERAL StrRaw(2) raw 42:Root[0000, 0]@63..73#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@73..74#ROOT2024
              LITERAL Char a 42:Root[0000, 0]@75..78#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@78..79#ROOT2024
              LITERAL Byte b 42:Root[0000, 0]@80..84#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@84..85#ROOT2024
              LITERAL CStr null 42:Root[0000, 0]@86..93#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              LITERAL Integer 1u16 42:Root[0000, 0]@0..4#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@4..5#ROOT2024
              LITERAL Integer 2_u32 42:Root[0000, 0]@6..11#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@11..12#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@13..14#ROOT2024
              LITERAL Integer 4i64 42:Root[0000, 0]@14..18#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@18..19#ROOT2024
              LITERAL Float 3.14f32 42:Root[0000, 0]@20..27#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@27..28#ROOT2024
              LITERAL Str hello bridge 42:Root[0000, 0]@29..43#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@43..44#ROOT2024
              LITERAL Str suffixedsuffix 42:Root[0000, 0]@45..61#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@61..62#ROOT2024
              LITERAL StrRaw(2) raw 42:Root[0000, 0]@63..73#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@73..74#ROOT2024
              LITERAL Char a 42:Root[0000, 0]@75..78#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@78..79#ROOT2024
              LITERAL Byte b 42:Root[0000, 0]@80..84#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@84..85#ROOT2024
              LITERAL CStr null 42:Root[0000, 0]@86..93#ROOT2024"#]] ,) ; }
};
}
