macro_rules! Empty {
    () => {
        # [doc = " An iterator that yields nothing."] # [derive (Clone , Debug)] pub struct Empty < T , E > (PhantomData < T > , PhantomData < E >) ;
    };
}

Empty!()