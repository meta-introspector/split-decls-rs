macro_rules! deps {
    () => {
        PrepareCheckout!();
        Repository!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        # [doc = " Access"] impl PrepareCheckout { # [doc = " Get access to the repository while the checkout isn't yet completed."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the checkout is completed and the [`Repository`] was already passed on to the caller."] pub fn repo (& self) -> & Repository { self . repo . as_ref () . expect ("present as checkout operation isn't complete") } }
    };
}

impl_96!();