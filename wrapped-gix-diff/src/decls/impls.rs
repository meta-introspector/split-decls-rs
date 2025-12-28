macro_rules! deps {
    () => {
        ResourceKind!();
    };
}

macro_rules! impls {
    () => {
        deps!();
        mod impls { use crate :: blob :: ResourceKind ; impl std :: fmt :: Display for ResourceKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str (match self { ResourceKind :: OldOrSource => "old" , ResourceKind :: NewOrDestination => "new" , }) } } }
    };
}

impls!();