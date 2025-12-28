macro_rules! deps {
    () => {
        MappedRef!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : ? Sized > MappedRef < 'a , K , T > { pub fn key (& self) -> & K { self . pair () . 0 } pub fn value (& self) -> & T { self . pair () . 1 } pub fn pair (& self) -> (& K , & T) { (self . k , self . v) } pub fn map < F , T2 > (self , f : F) -> MappedRef < 'a , K , T2 > where F : FnOnce (& T) -> & T2 , { MappedRef { _guard : self . _guard , k : self . k , v : f (self . v) , } } pub fn try_map < F , T2 : ? Sized > (self , f : F) -> Result < MappedRef < 'a , K , T2 > , Self > where F : FnOnce (& T) -> Option < & T2 > , { let v = match f (self . v) { Some (v) => v , None => return Err (self) , } ; let guard = self . _guard ; Ok (MappedRef { _guard : guard , k : self . k , v , }) } }
    };
}

impl_71!()