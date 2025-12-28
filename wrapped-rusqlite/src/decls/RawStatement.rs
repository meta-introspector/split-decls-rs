macro_rules! deps {
    () => {
        ParamIndexCache!();
    };
}

macro_rules! RawStatement {
    () => {
        deps!();
        # [derive (Debug)] pub struct RawStatement { ptr : * mut ffi :: sqlite3_stmt , cache : ParamIndexCache , # [cfg (feature = "cache")] statement_cache_key : Option < Arc < str > > , }
    };
}

RawStatement!()