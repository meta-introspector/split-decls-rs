macro_rules! context {
    () => {
        # [cfg (any (feature = "functions" , feature = "vtab"))] mod context ;
    };
}

context!()