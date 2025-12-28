macro_rules! compiler {
    () => {
        # [cfg (feature = "syntax")] mod compiler ;
    };
}

compiler!()