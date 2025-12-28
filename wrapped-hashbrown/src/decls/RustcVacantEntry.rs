macro_rules! deps {
    () => {
        RustcEntry!();
        RawTable!();
        HashMap!();
    };
}

macro_rules! RustcVacantEntry {
    () => {
        deps!();
        # [doc = " A view into a vacant entry in a `HashMap`."] # [doc = " It is part of the [`RustcEntry`] enum."] # [doc = ""] # [doc = " [`RustcEntry`]: enum.RustcEntry.html"] pub struct RustcVacantEntry < 'a , K , V , A = Global > where A : Allocator , { hash : u64 , key : K , table : & 'a mut RawTable < (K , V) , A > , }
    };
}

RustcVacantEntry!();