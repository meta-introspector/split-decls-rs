macro_rules! SqlFnArg {
    () => {
        # [doc = " n-th arg of an SQL scalar function"] pub struct SqlFnArg { idx : usize , }
    };
}

SqlFnArg!()