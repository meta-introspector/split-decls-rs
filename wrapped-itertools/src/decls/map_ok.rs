macro_rules! deps {
    () => {
        MapOk!();
        MapSpecialCaseFnOk!();
        MapSpecialCase!();
    };
}

macro_rules! map_ok {
    () => {
        deps!();
        # [doc = " Create a new `MapOk` iterator."] pub fn map_ok < I , F , T , U , E > (iter : I , f : F) -> MapOk < I , F > where I : Iterator < Item = Result < T , E > > , F : FnMut (T) -> U , { MapSpecialCase { iter , f : MapSpecialCaseFnOk (f) , } }
    };
}

map_ok!()