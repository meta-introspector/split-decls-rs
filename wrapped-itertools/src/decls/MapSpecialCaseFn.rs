macro_rules! MapSpecialCaseFn {
    () => {
        pub trait MapSpecialCaseFn < T > { type Out ; fn call (& mut self , t : T) -> Self :: Out ; }
    };
}

MapSpecialCaseFn!();