macro_rules! deps {
    () => {
        IndexType!();
    };
}

macro_rules! invalid_length_err {
    () => {
        deps!();
        pub fn invalid_length_err < Ix , E > (node_or_edge : & str , len : usize) -> E where E : Error , Ix : IndexType , { E :: custom (format_args ! ("invalid size: graph {} count {} exceeds index type maximum {}" , node_or_edge , len , < Ix as IndexType >:: max () . index ())) }
    };
}

invalid_length_err!();