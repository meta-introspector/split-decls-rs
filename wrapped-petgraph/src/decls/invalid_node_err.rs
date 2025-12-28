macro_rules! invalid_node_err {
    () => {
        pub fn invalid_node_err < E > (node_index : usize , len : usize) -> E where E : Error , { E :: custom (format_args ! ("invalid value: node index `{node_index}` does not exist in graph \
         with node bound {len}" ,)) }
    };
}

invalid_node_err!();