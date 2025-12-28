macro_rules! deps {
    () => {
        Iterator!();
        Convert!();
    };
}

macro_rules! convert {
    () => {
        deps!();
        # [doc = " Converts an `Iterator<Item = Result<T, E>>` into a `FallibleIterator<Item = T, Error = E>`."] # [inline] pub fn convert < T , E , I > (it : I) -> Convert < I > where I : iter :: Iterator < Item = Result < T , E > > , { Convert (it) }
    };
}

convert!();