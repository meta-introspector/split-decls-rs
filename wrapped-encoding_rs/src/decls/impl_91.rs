macro_rules! deps {
    () => {
        Gb18030Pending!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl Gb18030Pending { fn is_none (& self) -> bool { match * self { Gb18030Pending :: None => true , _ => false , } } fn count (& self) -> usize { match * self { Gb18030Pending :: None => 0 , Gb18030Pending :: One (_) => 1 , Gb18030Pending :: Two (_ , _) => 2 , Gb18030Pending :: Three (_ , _ , _) => 3 , } } }
    };
}

impl_91!();