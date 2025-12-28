macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! empty {
    () => {
        deps!();
        # [doc = " Returns an iterator over no items."] pub fn empty < T , E > () -> Empty < T , E > { Empty (PhantomData) }
    };
}

empty!()