macro_rules! deps {
    () => {
        CurlTransport!();
    };
}

macro_rules! factory {
    () => {
        deps!();
        fn factory (remote : & git2 :: Remote < '_ > , handle : Arc < Mutex < Easy > >) -> Result < Transport , Error > { Transport :: smart (remote , true , CurlTransport { handle : handle , base_url : Arc :: new (Mutex :: new (String :: new ())) , } ,) }
    };
}

factory!();