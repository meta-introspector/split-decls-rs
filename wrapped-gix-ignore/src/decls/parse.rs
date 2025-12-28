macro_rules! parse {
    () => {
        # [doc = " Parse git ignore patterns, line by line, from `bytes`."] # [doc = ""] # [doc = " If `support_precious` is `true`, we will parse `$` prefixed entries as precious."] # [doc = " This is backward-incompatible as files that actually start with `$` like `$houdini`"] # [doc = " will then not be ignored anymore, instead it ignores `houdini`."] pub fn parse (bytes : & [u8] , support_precious : bool) -> parse :: Lines < '_ > { parse :: Lines :: new (bytes , support_precious) }
    };
}

parse!()