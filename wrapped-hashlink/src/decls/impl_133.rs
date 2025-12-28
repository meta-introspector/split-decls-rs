macro_rules! deps {
    () => {
        LinkedHashMap!();
        LinkedHashSet!();
        DefaultHashBuilder!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < T : Hash + Eq > LinkedHashSet < T , DefaultHashBuilder > { # [inline] pub fn new () -> LinkedHashSet < T , DefaultHashBuilder > { LinkedHashSet { map : LinkedHashMap :: new () , } } # [inline] pub fn with_capacity (capacity : usize) -> LinkedHashSet < T , DefaultHashBuilder > { LinkedHashSet { map : LinkedHashMap :: with_capacity (capacity) , } } }
    };
}

impl_133!();