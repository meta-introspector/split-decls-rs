macro_rules! NextIfEqFn {
    () => {
        struct NextIfEqFn < 'a , T : ? Sized , Item > { expected : & 'a T , _next : PhantomData < Item > , }
    };
}

NextIfEqFn!();