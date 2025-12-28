macro_rules! deps {
    () => {
        ByteClasses!();
    };
}

macro_rules! ByteClassRepresentatives {
    () => {
        deps!();
        # [doc = " An iterator over representative bytes from each equivalence class."] # [doc = ""] # [doc = " This is created by the [`ByteClasses::representatives`] method."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the byte classes that this"] # [doc = " iterator was created from."] # [derive (Debug)] pub struct ByteClassRepresentatives < 'a > { classes : & 'a ByteClasses , cur_byte : usize , end_byte : Option < usize > , last_class : Option < u8 > , }
    };
}

ByteClassRepresentatives!()