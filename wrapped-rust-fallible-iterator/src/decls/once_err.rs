macro_rules! deps {
    () => {
        OnceErr!();
    };
}

macro_rules! once_err {
    () => {
        deps!();
        # [doc = " Creates an iterator that fails with a predetermined error exactly once."] pub fn once_err < T , E > (value : E) -> OnceErr < T , E > { OnceErr (PhantomData , Some (value)) }
    };
}

once_err!()