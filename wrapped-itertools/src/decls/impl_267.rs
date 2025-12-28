macro_rules! deps {
    () => {
        Groups!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < 'a , K , I , F > Debug for Groups < 'a , K , I , F > where K : Debug , I : Iterator + Debug , I :: Item : Debug , { debug_fmt_fields ! (Groups , parent) ; }
    };
}

impl_267!()