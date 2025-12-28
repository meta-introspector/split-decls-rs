macro_rules! FnOnce1 {
    () => {
        pub trait FnOnce1 < A > { type Output ; fn call_once (self , arg : A) -> Self :: Output ; }
    };
}

FnOnce1!();