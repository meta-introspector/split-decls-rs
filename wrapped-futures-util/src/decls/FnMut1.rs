macro_rules! deps {
    () => {
        FnOnce1!();
    };
}

macro_rules! FnMut1 {
    () => {
        deps!();
        pub trait FnMut1 < A > : FnOnce1 < A > { fn call_mut (& mut self , arg : A) -> Self :: Output ; }
    };
}

FnMut1!();