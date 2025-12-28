macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! RawEntryBuilderMut {
    () => {
        deps!();
        pub struct RawEntryBuilderMut < 'a , K , V , S > { map : & 'a mut LinkedHashMap < K , V , S > , }
    };
}

RawEntryBuilderMut!()