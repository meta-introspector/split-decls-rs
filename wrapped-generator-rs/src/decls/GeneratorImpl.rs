macro_rules! deps {
    () => {
        Stack!();
        Context!();
        Func!();
    };
}

macro_rules! GeneratorImpl {
    () => {
        deps!();
        # [doc = " `GeneratorImpl`"] # [repr (C)] struct GeneratorImpl < 'a , A , T > { context : Context , stack : Stack , para : Option < A > , ret : Option < T > , f : Option < Func > , phantom : PhantomData < & 'a T > , }
    };
}

GeneratorImpl!()