macro_rules! deps {
    () => {
        Deferred!();
        Bag!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl Drop for Bag { fn drop (& mut self) { for deferred in & mut self . deferreds [.. self . len] { let no_op = Deferred :: NO_OP ; let owned_deferred = mem :: replace (deferred , no_op) ; owned_deferred . call () ; } } }
    };
}

impl_105!()