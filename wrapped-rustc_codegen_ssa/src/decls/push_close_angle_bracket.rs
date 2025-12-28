macro_rules! push_close_angle_bracket {
    () => {
        fn push_close_angle_bracket (cpp_like_debuginfo : bool , output : & mut String) { if cpp_like_debuginfo && output . ends_with ('>') { output . push (' ') } ; output . push ('>') ; }
    };
}

push_close_angle_bracket!()