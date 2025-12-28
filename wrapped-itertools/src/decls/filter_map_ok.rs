macro_rules! deps {
    () => {
        FilterMapOk!();
    };
}

macro_rules! filter_map_ok {
    () => {
        deps!();
        # [doc = " Create a new `FilterMapOk` iterator."] pub fn filter_map_ok < I , F , T , U , E > (iter : I , f : F) -> FilterMapOk < I , F > where I : Iterator < Item = Result < T , E > > , F : FnMut (T) -> Option < U > , { FilterMapOk { iter , f } }
    };
}

filter_map_ok!()