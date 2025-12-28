macro_rules! deps {
    () => {
        Checkout!();
        Workers!();
        Tree!();
    };
}

macro_rules! impl_593 {
    () => {
        deps!();
        impl Checkout { # [doc = " The `checkout.workers` key."] pub const WORKERS : Workers = Workers :: new_with_validate ("workers" , & config :: Tree :: CHECKOUT , validate :: Workers) . with_deviation ("if unset, uses all cores instead of just one") ; }
    };
}

impl_593!();