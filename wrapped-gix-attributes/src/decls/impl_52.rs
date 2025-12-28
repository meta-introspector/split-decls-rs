macro_rules! deps {
    () => {
        RefMapKey!();
        RefMap!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T > RefMap < T > where T : Hash + Clone , { pub (crate) fn len (& self) -> usize { self . 0 . len () } pub (crate) fn insert (& mut self , value : & T) -> RefMapKey { let mut s = DefaultHasher :: new () ; value . hash (& mut s) ; let key = s . finish () ; match self . 0 . entry (key) { Entry :: Vacant (e) => { e . insert (value . clone ()) ; key } Entry :: Occupied (_) => key , } } pub (crate) fn insert_owned (& mut self , value : T) -> RefMapKey { let mut s = DefaultHasher :: new () ; value . hash (& mut s) ; let key = s . finish () ; match self . 0 . entry (key) { Entry :: Vacant (e) => { e . insert (value) ; key } Entry :: Occupied (_) => key , } } pub (crate) fn resolve (& self , key : RefMapKey) -> Option < & T > { self . 0 . get (& key) } }
    };
}

impl_52!()