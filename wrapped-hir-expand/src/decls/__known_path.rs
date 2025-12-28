macro_rules! __known_path {
    () => {
        # [macro_export] macro_rules ! __known_path { (core :: iter :: IntoIterator) => { } ; (core :: iter :: Iterator) => { } ; (core :: result :: Result) => { } ; (core :: option :: Option) => { } ; (core :: ops :: Range) => { } ; (core :: ops :: RangeFrom) => { } ; (core :: ops :: RangeFull) => { } ; (core :: ops :: RangeTo) => { } ; (core :: ops :: RangeToInclusive) => { } ; (core :: ops :: RangeInclusive) => { } ; (core :: future :: Future) => { } ; (core :: future :: IntoFuture) => { } ; (core :: fmt :: Debug) => { } ; (std :: fmt :: format) => { } ; (core :: ops :: Try) => { } ; (core :: convert :: From) => { } ; (core :: convert :: TryFrom) => { } ; (core :: str :: FromStr) => { } ; ($ path : path) => { compile_error ! ("Please register your known path in the path module") } ; }
    };
}

__known_path!()