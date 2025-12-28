macro_rules! deps {
    () => {
        ByteClasses!();
    };
}

macro_rules! ByteClassIter {
    () => {
        deps!();
        # [doc = " An iterator over each equivalence class."] # [doc = ""] # [doc = " The last element in this iterator always corresponds to [`Unit::eoi`]."] # [doc = ""] # [doc = " This is created by the [`ByteClasses::iter`] method."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the byte classes that this"] # [doc = " iterator was created from."] # [derive (Debug)] pub struct ByteClassIter < 'a > { classes : & 'a ByteClasses , i : usize , }
    };
}

ByteClassIter!()