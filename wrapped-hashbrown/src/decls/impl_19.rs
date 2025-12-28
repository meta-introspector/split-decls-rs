macro_rules! deps {
    () => {
        Tag!();
        TagSliceExt!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl TagSliceExt for [Tag] { # [inline] fn fill_tag (& mut self , tag : Tag) { unsafe { self . as_mut_ptr () . write_bytes (tag . 0 , self . len ()) } } }
    };
}

impl_19!();