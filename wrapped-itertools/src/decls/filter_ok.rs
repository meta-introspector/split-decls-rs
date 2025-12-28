macro_rules! deps {
    () => {
        FilterOk!();
    };
}

macro_rules! filter_ok {
    () => {
        deps!();
        # [doc = " Create a new `FilterOk` iterator."] pub fn filter_ok < I , F , T , E > (iter : I , f : F) -> FilterOk < I , F > where I : Iterator < Item = Result < T , E > > , F : FnMut (& T) -> bool , { FilterOk { iter , f } }
    };
}

filter_ok!();