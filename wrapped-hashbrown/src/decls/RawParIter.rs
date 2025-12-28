macro_rules! deps {
    () => {
        RawIterRange!();
    };
}

macro_rules! RawParIter {
    () => {
        deps!();
        # [doc = " Parallel iterator which returns a raw pointer to every full bucket in the table."] pub struct RawParIter < T > { iter : RawIterRange < T > , }
    };
}

RawParIter!()