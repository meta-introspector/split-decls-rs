// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a , K : Hash + PhfHash + Eq + FmtConst > OrderedMap < 'a , K > { # [doc = " Constructs a enw `phf::OrderedMap` builder."] pub fn new () -> Self { OrderedMap { keys : vec ! [] , values : vec ! [] , path : Cow :: Borrowed ("::phf") , } } # [doc = " Set the path to the `phf` crate from the global namespace"] pub fn phf_path (& mut self , path : impl Into < Cow < 'a , str > >) -> & mut Self { self . path = path . into () ; self } # [doc = " Adds an entry to the builder."] # [doc = ""] # [doc = " `value` will be written exactly as provided in the constructed source."] pub fn entry (& mut self , key : K , value : impl Into < Cow < 'a , str > >) -> & mut Self { self . keys . push (key) ; self . values . push (value . into ()) ; self } # [doc = " Calculate the hash parameters and return a struct implementing"] # [doc = " [`Display`](::std::fmt::Display) which will print the constructed"] # [doc = " `phf::OrderedMap`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if there are any duplicate keys."] pub fn build (& self) -> DisplayOrderedMap < '_ , K > { let mut set = HashSet :: new () ; for key in & self . keys { if ! set . insert (key) { panic ! ("duplicate key `{}`" , Delegate (key)) ; } } let state = phf_generator :: generate_hash (& self . keys) ; DisplayOrderedMap { state , path : & self . path , keys : & self . keys , values : & self . values , } } }
};
}
