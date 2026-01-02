mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{big, { 
                getname!(big);
                getsrc!(big);
                getpath!(big);
                get_deps!(big);
                get_crates!(big);
                mkinclude!(big);
                 
            }}
mkmod!{env, { 
                getname!(env);
                getsrc!(env);
                getpath!(env);
                get_deps!(env);
                get_crates!(env);
                mkinclude!(env);
                 
            }}
mkmod!{feature_detect, { 
                getname!(feature_detect);
                getsrc!(feature_detect);
                getpath!(feature_detect);
                get_deps!(feature_detect);
                get_crates!(feature_detect);
                mkinclude!(feature_detect);
                 
            }}
mkmod!{float_traits, { 
                getname!(float_traits);
                getsrc!(float_traits);
                getpath!(float_traits);
                get_deps!(float_traits);
                get_crates!(float_traits);
                mkinclude!(float_traits);
                 
            }}
mkmod!{hex_float, { 
                getname!(hex_float);
                getsrc!(hex_float);
                getpath!(hex_float);
                get_deps!(hex_float);
                get_crates!(hex_float);
                mkinclude!(hex_float);
                 
            }}
mkmod!{int_traits, { 
                getname!(int_traits);
                getsrc!(int_traits);
                getpath!(int_traits);
                get_deps!(int_traits);
                get_crates!(int_traits);
                mkinclude!(int_traits);
                 
            }}
mkuse!{# [allow (unused_imports)] pub use big :: { i256 , u256 } ;}
mkuse!{# [allow (unused_imports , clippy :: single_component_path_imports)] pub (crate) use cfg_if ;}
mkuse!{pub use env :: { FpResult , Round , Status } ;}
mkuse!{# [allow (unused_imports)] pub use float_traits :: { DFloat , Float , HFloat , IntTy } ;}
mkuse!{pub (crate) use float_traits :: { f32_from_bits , f64_from_bits } ;}
mkuse!{# [cfg (any (test , feature = "unstable-public-internals"))] pub use hex_float :: Hexf ;}
mkuse!{# [cfg (f16_enabled)] # [allow (unused_imports)] pub use hex_float :: hf16 ;}
mkuse!{# [cfg (f128_enabled)] # [allow (unused_imports)] pub use hex_float :: hf128 ;}
mkuse!{# [allow (unused_imports)] pub use hex_float :: { hf32 , hf64 } ;}
mkuse!{pub use int_traits :: { CastFrom , CastInto , DInt , HInt , Int , MinInt } ;}

macro_rules! cold_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function cold_path in module {}", module_path!());
    };
}

mkfn!{
    cold_path_introspect!();
    # [doc = " Hint to the compiler that the current path is cold."] pub fn cold_path () { # [cfg (intrinsics_enabled)] core :: intrinsics :: cold_path () ; }
}