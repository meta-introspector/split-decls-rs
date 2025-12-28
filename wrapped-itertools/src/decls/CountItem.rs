macro_rules! CountItem {
    () => {
        pub trait CountItem < T > { type CItem ; fn new (t : T) -> Self :: CItem ; }
    };
}

CountItem!()