macro_rules! Once {
    () => {
        # [doc = " An iterator that yields something exactly once."] # [derive (Clone , Debug)] pub struct Once < T , E > (Option < T > , PhantomData < E >) ;
    };
}

Once!()