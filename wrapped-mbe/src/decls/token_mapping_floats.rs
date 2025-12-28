macro_rules! token_mapping_floats {
    () => {
        # [test] fn token_mapping_floats () { check (Edition :: CURRENT , Edition :: CURRENT , r#"
($($tt:tt)*) => {
    $($tt)*
};
"# , r#"
fn main() {
    1;
    1.0;
    ((1,),).0.0;
    let x = 1;
}
"# , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..63#ROOT2024 1:Root[0000, 0]@0..63#ROOT2024
              IDENT   fn 1:Root[0000, 0]@1..3#ROOT2024
              IDENT   main 1:Root[0000, 0]@4..8#ROOT2024
              SUBTREE () 1:Root[0000, 0]@8..9#ROOT2024 1:Root[0000, 0]@9..10#ROOT2024
              SUBTREE {} 1:Root[0000, 0]@11..12#ROOT2024 1:Root[0000, 0]@61..62#ROOT2024
                LITERAL Integer 1 1:Root[0000, 0]@17..18#ROOT2024
                PUNCH   ; [alone] 1:Root[0000, 0]@18..19#ROOT2024
                LITERAL Float 1.0 1:Root[0000, 0]@24..27#ROOT2024
                PUNCH   ; [alone] 1:Root[0000, 0]@27..28#ROOT2024
                SUBTREE () 1:Root[0000, 0]@33..34#ROOT2024 1:Root[0000, 0]@39..40#ROOT2024
                  SUBTREE () 1:Root[0000, 0]@34..35#ROOT2024 1:Root[0000, 0]@37..38#ROOT2024
                    LITERAL Integer 1 1:Root[0000, 0]@35..36#ROOT2024
                    PUNCH   , [alone] 1:Root[0000, 0]@36..37#ROOT2024
                  PUNCH   , [alone] 1:Root[0000, 0]@38..39#ROOT2024
                PUNCH   . [alone] 1:Root[0000, 0]@40..41#ROOT2024
                LITERAL Float 0.0 1:Root[0000, 0]@41..44#ROOT2024
                PUNCH   ; [alone] 1:Root[0000, 0]@44..45#ROOT2024
                IDENT   let 1:Root[0000, 0]@50..53#ROOT2024
                IDENT   x 1:Root[0000, 0]@54..55#ROOT2024
                PUNCH   = [alone] 1:Root[0000, 0]@56..57#ROOT2024
                LITERAL Integer 1 1:Root[0000, 0]@58..59#ROOT2024
                PUNCH   ; [alone] 1:Root[0000, 0]@59..60#ROOT2024

            fn main(){
                1;
                1.0;
                ((1,),).0.0;
                let x = 1;
            }"#]] ,) ; }
    };
}

token_mapping_floats!();