macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Env { pub fn set (& mut self , env : & str , value : impl Into < String >) { self . entries . insert (env . to_owned () , value . into ()) ; } pub fn get (& self , env : & str) -> Option < String > { self . entries . get (env) . cloned () } pub fn extend_from_other (& mut self , other : & Env) { self . entries . extend (other . entries . iter () . map (| (x , y) | (x . to_owned () , y . to_owned ()))) ; } pub fn is_empty (& self) -> bool { self . entries . is_empty () } pub fn insert (& mut self , k : impl Into < String > , v : impl Into < String >) -> Option < String > { self . entries . insert (k . into () , v . into ()) } pub fn contains_key (& self , arg : & str) -> bool { self . entries . contains_key (arg) } }
    };
}

impl_56!();