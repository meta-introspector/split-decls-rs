macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl < 'a , I > Drop for Chunk < 'a , I > where I : Iterator , I :: Item : 'a , { fn drop (& mut self) { self . parent . drop_group (self . index) ; } }
    };
}

impl_283!()