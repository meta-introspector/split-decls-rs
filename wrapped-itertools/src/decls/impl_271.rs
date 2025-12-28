macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < 'a , K , I , F > Debug for Group < 'a , K , I , F > where K : Debug , I : Iterator + Debug , I :: Item : Debug , { debug_fmt_fields ! (Group , parent , index , first) ; }
    };
}

impl_271!()