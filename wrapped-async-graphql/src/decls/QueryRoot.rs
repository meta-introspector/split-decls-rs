macro_rules! QueryRoot {
    () => {
        pub (crate) struct QueryRoot < T > { pub (crate) inner : T , }
    };
}

QueryRoot!();