macro_rules! deps {
    () => {
        ByteClasses!();
    };
}

macro_rules! ByteClassElements {
    () => {
        deps!();
        # [doc = " An iterator over all elements in a specific equivalence class."] # [derive (Debug)] pub (crate) struct ByteClassElements < 'a > { classes : & 'a ByteClasses , class : u8 , bytes : core :: ops :: RangeInclusive < u8 > , }
    };
}

ByteClassElements!()