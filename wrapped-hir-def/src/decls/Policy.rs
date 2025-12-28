macro_rules! deps {
    () => {
        DynMap!();
    };
}

macro_rules! Policy {
    () => {
        deps!();
        pub trait Policy { type K ; type V ; fn insert (map : & mut DynMap , key : Self :: K , value : Self :: V) ; fn get < 'a > (map : & 'a DynMap , key : & Self :: K) -> Option < & 'a Self :: V > ; fn is_empty (map : & DynMap) -> bool ; }
    };
}

Policy!()