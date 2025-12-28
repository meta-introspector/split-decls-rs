macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_618 {
    () => {
        deps!();
        impl < 'a , K : 'a , V : 'a > Entry < 'a , K , V > { pub (crate) fn or_insert (self , default : V) -> & 'a mut V { match self { Entry :: Occupied (entry) => & mut entry . v . values [entry . index] , Entry :: Vacant (entry) => { entry . v . keys . push (entry . key) ; entry . v . values . push (default) ; entry . v . values . last_mut () . unwrap () } } } pub (crate) fn or_insert_with < F : FnOnce () -> V > (self , default : F) -> & 'a mut V { match self { Entry :: Occupied (entry) => & mut entry . v . values [entry . index] , Entry :: Vacant (entry) => { entry . v . keys . push (entry . key) ; entry . v . values . push (default ()) ; entry . v . values . last_mut () . unwrap () } } } }
    };
}

impl_618!();