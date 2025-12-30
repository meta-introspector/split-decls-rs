// Generated macro for test_fn_like_fn_like_span_ops (function)
macro_rules! Depcrate_teststest_fn_like_fn_like_span_ops {
() => {
// Module: crate::tests
// Provides: {"test_fn_like_fn_like_span_ops"}
// Dependencies: {}
# [test] fn test_fn_like_fn_like_span_ops () { assert_expand ("fn_like_span_ops" , "set_def_site resolved_at_def_site start_span" , expect ! [[r#"
            SUBTREE $$ 1 1
              IDENT   set_def_site 1
              IDENT   resolved_at_def_site 1
              IDENT   start_span 1



            SUBTREE $$ 1 1
              IDENT   set_def_site 0
              IDENT   resolved_at_def_site 1
              IDENT   start_span 1"#]] , expect ! [[r#"
            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   set_def_site 42:Root[0000, 0]@0..12#ROOT2024
              IDENT   resolved_at_def_site 42:Root[0000, 0]@13..33#ROOT2024
              IDENT   start_span 42:Root[0000, 0]@34..44#ROOT2024



            SUBTREE $$ 42:Root[0000, 0]@0..100#ROOT2024 42:Root[0000, 0]@0..100#ROOT2024
              IDENT   set_def_site 41:Root[0000, 0]@0..150#ROOT2024
              IDENT   resolved_at_def_site 42:Root[0000, 0]@13..33#ROOT2024
              IDENT   start_span 42:Root[0000, 0]@34..34#ROOT2024"#]] ,) ; }
};
}
