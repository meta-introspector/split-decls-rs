macro_rules! vtablog {
    () => {
        # [cfg (all (test , feature = "modern_sqlite"))] mod vtablog ;
    };
}

vtablog!()