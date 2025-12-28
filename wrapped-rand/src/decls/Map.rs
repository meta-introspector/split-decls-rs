macro_rules! deps {
    () => {
        Distribution!();
    };
}

macro_rules! Map {
    () => {
        deps!();
        # [doc = " A [`Distribution`] which maps sampled values to type `S`"] # [doc = ""] # [doc = " This `struct` is created by the [`Distribution::map`] method."] # [doc = " See its documentation for more."] # [derive (Debug)] pub struct Map < D , F , T , S > { distr : D , func : F , phantom : core :: marker :: PhantomData < fn (T) -> S > , }
    };
}

Map!();