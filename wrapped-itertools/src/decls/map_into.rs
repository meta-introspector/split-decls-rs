macro_rules! deps {
    () => {
        MapSpecialCase!();
        MapSpecialCaseFnInto!();
        MapInto!();
    };
}

macro_rules! map_into {
    () => {
        deps!();
        # [doc = " Create a new [`MapInto`] iterator."] pub fn map_into < I , R > (iter : I) -> MapInto < I , R > { MapSpecialCase { iter , f : MapSpecialCaseFnInto (PhantomData) , } }
    };
}

map_into!()