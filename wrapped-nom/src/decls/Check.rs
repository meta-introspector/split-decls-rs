macro_rules! Check {
    () => {
        # [doc = " Applies the parser, but do not a produce a value"] # [doc = ""] # [doc = " This has the effect of greatly reducing the amount of code generated and the"] # [doc = " parser memory usage. Some combinators check for an error in a child parser but"] # [doc = " discard the error, and for those, using [Check] makes sure the error is not"] # [doc = " even generated, only the fact that an error happened remains"] pub struct Check ;
    };
}

Check!();