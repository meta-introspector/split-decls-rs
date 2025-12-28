macro_rules! deps {
    () => {
        Configuration!();
    };
}

macro_rules! generate {
    () => {
        deps!();
        const fn generate < E , I , L > () -> Configuration < E , I , L > { Configuration { _e : PhantomData , _i : PhantomData , _l : PhantomData , } }
    };
}

generate!()