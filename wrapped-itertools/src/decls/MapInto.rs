macro_rules! deps {
    () => {
        MapSpecialCaseFnInto!();
        MapSpecialCase!();
    };
}

macro_rules! MapInto {
    () => {
        deps!();
        # [doc = " An iterator adapter to apply `Into` conversion to each element."] # [doc = ""] # [doc = " See [`.map_into()`](crate::Itertools::map_into) for more information."] pub type MapInto < I , R > = MapSpecialCase < I , MapSpecialCaseFnInto < R > > ;
    };
}

MapInto!();