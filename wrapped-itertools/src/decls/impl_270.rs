macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < 'a , K , I , F > Drop for Group < 'a , K , I , F > where I : Iterator , I :: Item : 'a , { fn drop (& mut self) { self . parent . drop_group (self . index) ; } }
    };
}

impl_270!()