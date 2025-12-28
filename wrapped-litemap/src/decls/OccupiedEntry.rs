macro_rules! OccupiedEntry {
    () => {
        # [doc = " A view into an occupied entry in a `LiteMap`."] pub struct OccupiedEntry < 'a , K , V , S > { map : & 'a mut LiteMap < K , V , S > , index : usize , }
    };
}

OccupiedEntry!();