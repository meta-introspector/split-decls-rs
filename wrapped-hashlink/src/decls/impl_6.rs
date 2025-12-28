macro_rules! deps {
    () => {
        LinkedHashMap!();
        RawEntryBuilderMut!();
        RawEntryBuilder!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < K , V , S > LinkedHashMap < K , V , S > where S : BuildHasher , { # [inline] pub fn raw_entry (& self) -> RawEntryBuilder < '_ , K , V , S > { RawEntryBuilder { map : self } } # [inline] pub fn raw_entry_mut (& mut self) -> RawEntryBuilderMut < '_ , K , V , S > { RawEntryBuilderMut { map : self } } }
    };
}

impl_6!();