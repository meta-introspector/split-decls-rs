macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! hashmap {
    () => {
        deps!();
        # [cfg (not (feature = "hp-hashmap"))] mod hashmap { use std :: collections :: HashMap ; use parking_lot :: Mutex ; pub struct Concurrent < K , V > { inner : Mutex < HashMap < K , V > > , } impl < K , V > Default for Concurrent < K , V > where K : Eq + std :: hash :: Hash , { fn default () -> Self { Concurrent { inner : Default :: default () , } } } impl < K , V > Concurrent < K , V > where K : Eq + std :: hash :: Hash + Clone , { pub fn insert (& self , key : K , value : V) -> Option < V > { self . inner . lock () . insert (key , value) } pub fn remove (& self , key : & K) -> Option < (K , V) > { self . inner . lock () . remove (key) . map (| v | (key . clone () , v)) } pub fn for_each < F > (& self , cb : F) where Self : Sized , F : FnMut (& mut V) , { if let Some (mut guard) = self . inner . try_lock () { guard . values_mut () . for_each (cb) ; } } } }
    };
}

hashmap!()