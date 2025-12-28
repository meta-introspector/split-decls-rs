macro_rules! deps {
    () => {
        RustcEntry!();
        HashMap!();
        Bucket!();
        RawTable!();
    };
}

macro_rules! RustcOccupiedEntry {
    () => {
        deps!();
        # [doc = " A view into an occupied entry in a `HashMap`."] # [doc = " It is part of the [`RustcEntry`] enum."] # [doc = ""] # [doc = " [`RustcEntry`]: enum.RustcEntry.html"] pub struct RustcOccupiedEntry < 'a , K , V , A = Global > where A : Allocator , { elem : Bucket < (K , V) > , table : & 'a mut RawTable < (K , V) , A > , }
    };
}

RustcOccupiedEntry!()