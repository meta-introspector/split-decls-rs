macro_rules! invalid_hole_err {
    () => {
        pub fn invalid_hole_err < E > (node_index : usize) -> E where E : Error , { E :: custom (format_args ! ("invalid value: node hole `{node_index}` is not allowed." ,)) }
    };
}

invalid_hole_err!()