macro_rules! deps {
    () => {
        MappedRefMut!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a , K : Eq + Hash , T : ? Sized > MappedRefMut < 'a , K , T > { pub fn key (& self) -> & K { self . pair () . 0 } pub fn value (& self) -> & T { self . pair () . 1 } pub fn value_mut (& mut self) -> & mut T { self . pair_mut () . 1 } pub fn pair (& self) -> (& K , & T) { (self . k , self . v) } pub fn pair_mut (& mut self) -> (& K , & mut T) { (self . k , self . v) } pub fn map < F , T2 : ? Sized > (self , f : F) -> MappedRefMut < 'a , K , T2 > where F : FnOnce (& mut T) -> & mut T2 , { MappedRefMut { _guard : self . _guard , k : self . k , v : f (self . v) , } } pub fn try_map < F , T2 : ? Sized > (self , f : F) -> Result < MappedRefMut < 'a , K , T2 > , Self > where F : FnOnce (& mut T) -> Option < & mut T2 > , { let v = match f (unsafe { & mut * (self . v as * mut _) }) { Some (v) => v , None => return Err (self) , } ; let guard = self . _guard ; let k = self . k ; Ok (MappedRefMut { _guard : guard , k , v , }) } }
    };
}

impl_77!()