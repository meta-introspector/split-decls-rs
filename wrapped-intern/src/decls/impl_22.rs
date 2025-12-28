macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Clone for Symbol { fn clone (& self) -> Self { Self { repr : increase_arc_refcount (self . repr) } } }
    };
}

impl_22!();