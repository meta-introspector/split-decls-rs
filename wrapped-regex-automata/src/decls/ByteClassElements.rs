macro_rules! deps {
    () => {
        ByteClasses!();
        Unit!();
    };
}

macro_rules! ByteClassElements {
    () => {
        deps!();
        # [doc = " An iterator over all elements in an equivalence class."] # [doc = ""] # [doc = " This is created by the [`ByteClasses::elements`] method."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the byte classes that this"] # [doc = " iterator was created from."] # [derive (Debug)] pub struct ByteClassElements < 'a > { classes : & 'a ByteClasses , class : Unit , byte : usize , }
    };
}

ByteClassElements!();