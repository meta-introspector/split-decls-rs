macro_rules! deps {
    () => {
        Remote!();
    };
}

macro_rules! Refspecs {
    () => {
        deps!();
        # [doc = " An iterator over the refspecs that a remote contains."] pub struct Refspecs < 'remote > { range : Range < usize > , remote : & 'remote Remote < 'remote > , }
    };
}

Refspecs!()