macro_rules! deps {
    () => {
        RustcVacantEntry!();
        HashMap!();
        RustcOccupiedEntry!();
    };
}

macro_rules! RustcEntry {
    () => {
        deps!();
        # [doc = " A view into a single entry in a map, which may either be vacant or occupied."] # [doc = ""] # [doc = " This `enum` is constructed from the [`rustc_entry`] method on [`HashMap`]."] # [doc = ""] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = " [`rustc_entry`]: struct.HashMap.html#method.rustc_entry"] pub enum RustcEntry < 'a , K , V , A = Global > where A : Allocator , { # [doc = " An occupied entry."] Occupied (RustcOccupiedEntry < 'a , K , V , A >) , # [doc = " A vacant entry."] Vacant (RustcVacantEntry < 'a , K , V , A >) , }
    };
}

RustcEntry!()