macro_rules! test_fn_like_macro_negative_literals {
    () => {
        # [test] fn test_fn_like_macro_negative_literals () { assert_expand ("fn_like_clone_tokens" , r###"-1u16, - 2_u32, -3.14f32, - 2.7"### , expect ! [[r#"
            SUBTREE $$ 1 1
              PUNCH   - [alone] 1
              LITERAL Integer 1u16 1
              PUNCH   , [alone] 1
              PUNCH   - [alone] 1
              LITERAL Integer 2_u32 1
              PUNCH   , [alone] 1
              PUNCH   - [alone] 1
              LITERAL Float 3.14f32 1
              PUNCH   , [alone] 1
              PUNCH   - [alone] 1
              LITERAL Float 2.7 1



            SUBTREE $$ 1 1
              PUNCH   - [alone] 1
              LITERAL Integer 1u16 1
              PUNCH   , [alone] 1
              PUNCH   - [alone] 1
              LITERAL Integer 2_u32 1
              PUNCH   , [alone] 1
              PUNCH   - [alone] 1
              LITERAL Float 3.14f32 1
              PUNCH   , [alone] 1
              PUNCH   - [alone] 1
              LITERAL Float 2.7 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@0..1#ROOT2024
              LITERAL Integer 1u16 42:Root[0000, 0]@1..5#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@5..6#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@7..8#ROOT2024
              LITERAL Integer 2_u32 42:Root[0000, 0]@9..14#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@14..15#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@16..17#ROOT2024
              LITERAL Float 3.14f32 42:Root[0000, 0]@17..24#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@24..25#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@26..27#ROOT2024
              LITERAL Float 2.7 42:Root[0000, 0]@28..31#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@0..1#ROOT2024
              LITERAL Integer 1u16 42:Root[0000, 0]@1..5#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@5..6#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@7..8#ROOT2024
              LITERAL Integer 2_u32 42:Root[0000, 0]@9..14#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@14..15#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@16..17#ROOT2024
              LITERAL Float 3.14f32 42:Root[0000, 0]@17..24#ROOT2024
              PUNCH   , [alone] 42:Root[0000, 0]@24..25#ROOT2024
              PUNCH   - [alone] 42:Root[0000, 0]@26..27#ROOT2024
              LITERAL Float 2.7 42:Root[0000, 0]@28..31#ROOT2024"#]] ,) ; }
    };
}

test_fn_like_macro_negative_literals!();