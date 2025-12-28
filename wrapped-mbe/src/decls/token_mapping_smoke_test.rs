macro_rules! token_mapping_smoke_test {
    () => {
        # [test] fn token_mapping_smoke_test () { check (Edition :: CURRENT , Edition :: CURRENT , r#"
( struct $ident:ident ) => {
    struct $ident {
        map: ::std::collections::HashSet<()>,
    }
};
"# , r#"
struct MyTraitMap2
"# , expect ! [[r#"
            SUBTREE $$ 1:Root[0000, 0]@0..20#ROOT2024 1:Root[0000, 0]@0..20#ROOT2024
              IDENT   struct 0:Root[0000, 0]@34..40#ROOT2024
              IDENT   MyTraitMap2 1:Root[0000, 0]@8..19#ROOT2024
              SUBTREE {} 0:Root[0000, 0]@48..49#ROOT2024 0:Root[0000, 0]@100..101#ROOT2024
                IDENT   map 0:Root[0000, 0]@58..61#ROOT2024
                PUNCH   : [alone] 0:Root[0000, 0]@61..62#ROOT2024
                PUNCH   : [joint] 0:Root[0000, 0]@63..64#ROOT2024
                PUNCH   : [alone] 0:Root[0000, 0]@64..65#ROOT2024
                IDENT   std 0:Root[0000, 0]@65..68#ROOT2024
                PUNCH   : [joint] 0:Root[0000, 0]@68..69#ROOT2024
                PUNCH   : [alone] 0:Root[0000, 0]@69..70#ROOT2024
                IDENT   collections 0:Root[0000, 0]@70..81#ROOT2024
                PUNCH   : [joint] 0:Root[0000, 0]@81..82#ROOT2024
                PUNCH   : [alone] 0:Root[0000, 0]@82..83#ROOT2024
                IDENT   HashSet 0:Root[0000, 0]@83..90#ROOT2024
                PUNCH   < [alone] 0:Root[0000, 0]@90..91#ROOT2024
                SUBTREE () 0:Root[0000, 0]@91..92#ROOT2024 0:Root[0000, 0]@92..93#ROOT2024
                PUNCH   > [joint] 0:Root[0000, 0]@93..94#ROOT2024
                PUNCH   , [alone] 0:Root[0000, 0]@94..95#ROOT2024

            struct MyTraitMap2 {
                map: ::std::collections::HashSet<()>,
            }"#]] ,) ; }
    };
}

token_mapping_smoke_test!();