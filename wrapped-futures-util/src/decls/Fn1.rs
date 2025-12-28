macro_rules! deps {
    () => {
        FnMut1!();
    };
}

macro_rules! Fn1 {
    () => {
        deps!();
        # [allow (dead_code , unreachable_pub)] pub trait Fn1 < A > : FnMut1 < A > { fn call (& self , arg : A) -> Self :: Output ; }
    };
}

Fn1!()