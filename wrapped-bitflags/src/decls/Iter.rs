macro_rules! deps {
    () => {
        IterNames!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = "\nAn iterator over flags values.\n\nThis iterator will yield flags values for contained, defined flags first, with any remaining bits yielded\nas a final flags value.\n"] pub struct Iter < B : 'static > { inner : IterNames < B > , done : bool , }
    };
}

Iter!()