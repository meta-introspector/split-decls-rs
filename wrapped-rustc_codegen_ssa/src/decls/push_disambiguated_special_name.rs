macro_rules! push_disambiguated_special_name {
    () => {
        fn push_disambiguated_special_name (label : & str , disambiguator : u32 , cpp_like_debuginfo : bool , output : & mut String ,) { if cpp_like_debuginfo { write ! (output , "{label}${disambiguator}") . unwrap () ; } else { write ! (output , "{{{label}#{disambiguator}}}") . unwrap () ; } }
    };
}

push_disambiguated_special_name!();