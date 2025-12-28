macro_rules! deps {
    () => {
        UnlockNotification!();
    };
}

macro_rules! impl_518 {
    () => {
        deps!();
        impl UnlockNotification { fn new () -> Self { Self { cond : Condvar :: new () , mutex : Mutex :: new (false) , } } fn fired (& self) { let mut flag = unpoison (self . mutex . lock ()) ; * flag = true ; self . cond . notify_one () ; } fn wait (& self) { let mut fired = unpoison (self . mutex . lock ()) ; while ! * fired { fired = unpoison (self . cond . wait (fired)) ; } } }
    };
}

impl_518!();