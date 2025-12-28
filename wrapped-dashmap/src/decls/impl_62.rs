macro_rules! deps {
    () => {
        RwLockReadGuardDetached!();
        MappedRef!();
        Ref!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , V > Ref < 'a , K , V > { pub (crate) fn new (guard : RwLockReadGuardDetached < 'a > , k : & 'a K , v : & 'a V) -> Self { Self { _guard : guard , k , v , } } pub fn key (& self) -> & K { self . pair () . 0 } pub fn value (& self) -> & V { self . pair () . 1 } pub fn pair (& self) -> (& K , & V) { (self . k , self . v) } pub fn map < F , T : ? Sized > (self , f : F) -> MappedRef < 'a , K , T > where F : FnOnce (& V) -> & T , { MappedRef { _guard : self . _guard , k : self . k , v : f (self . v) , } } pub fn try_map < F , T : ? Sized > (self , f : F) -> Result < MappedRef < 'a , K , T > , Self > where F : FnOnce (& V) -> Option < & T > , { if let Some (v) = f (self . v) { Ok (MappedRef { _guard : self . _guard , k : self . k , v , }) } else { Err (self) } } }
    };
}

impl_62!()