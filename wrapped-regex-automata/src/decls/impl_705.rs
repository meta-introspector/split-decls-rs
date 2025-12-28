macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! impl_705 {
    () => {
        deps!();
        impl < T , F > Pool < T , F > { # [doc = " Create a new pool. The given closure is used to create values in"] # [doc = " the pool when necessary."] pub fn new (create : F) -> Pool < T , F > { Pool (alloc :: boxed :: Box :: new (inner :: Pool :: new (create))) } }
    };
}

impl_705!();