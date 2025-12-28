macro_rules! Empty {
    () => {
        # [doc = " An iterator over no items."] pub struct Empty < T , E > (PhantomData < (T , E) >) ;
    };
}

Empty!()