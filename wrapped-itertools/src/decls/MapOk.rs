macro_rules! deps {
    () => {
        MapSpecialCase!();
        MapSpecialCaseFnOk!();
    };
}

macro_rules! MapOk {
    () => {
        deps!();
        # [doc = " An iterator adapter to apply a transformation within a nested `Result::Ok`."] # [doc = ""] # [doc = " See [`.map_ok()`](crate::Itertools::map_ok) for more information."] pub type MapOk < I , F > = MapSpecialCase < I , MapSpecialCaseFnOk < F > > ;
    };
}

MapOk!();