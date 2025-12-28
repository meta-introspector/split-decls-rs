macro_rules! deps {
    () => {
        Rows!();
    };
}

macro_rules! Map {
    () => {
        deps!();
        # [doc = " `F` is used to transform the _streaming_ iterator into a _fallible_"] # [doc = " iterator."] # [must_use = "iterators are lazy and do nothing unless consumed"] pub struct Map < 'stmt , F > { rows : Rows < 'stmt > , f : F , }
    };
}

Map!();