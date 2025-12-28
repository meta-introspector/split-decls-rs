macro_rules! VacantEntry {
    () => {
        # [doc = " A view into a vacant entry in a `LiteMap`."] pub struct VacantEntry < 'a , K , V , S > { map : & 'a mut LiteMap < K , V , S > , key : K , index : usize , }
    };
}

VacantEntry!()