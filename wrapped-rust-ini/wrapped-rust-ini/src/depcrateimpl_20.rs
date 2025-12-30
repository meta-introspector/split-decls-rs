// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > SectionSetter < 'a > { fn new (ini : & 'a mut Ini , section_name : Option < String >) -> SectionSetter < 'a > { SectionSetter { ini , section_name } } # [doc = " Set (replace) key-value pair in this section (all with the same name)"] pub fn set < 'b , K , V > (& 'b mut self , key : K , value : V) -> & 'b mut SectionSetter < 'a > where K : Into < String > , V : Into < String > , 'a : 'b , { self . ini . entry (self . section_name . clone ()) . or_insert_with (Default :: default) . insert (key , value) ; self } # [doc = " Add (append) key-value pair in this section"] pub fn add < 'b , K , V > (& 'b mut self , key : K , value : V) -> & 'b mut SectionSetter < 'a > where K : Into < String > , V : Into < String > , 'a : 'b , { self . ini . entry (self . section_name . clone ()) . or_insert_with (Default :: default) . append (key , value) ; self } # [doc = " Delete the first entry in this section with `key`"] pub fn delete < 'b , K > (& 'b mut self , key : & K) -> & 'b mut SectionSetter < 'a > where K : AsRef < str > , 'a : 'b , { for prop in self . ini . section_all_mut (self . section_name . as_ref ()) { prop . remove (key) ; } self } # [doc = " Get the entry in this section with `key`"] pub fn get < K : AsRef < str > > (& 'a self , key : K) -> Option < & 'a str > { self . ini . section (self . section_name . as_ref ()) . and_then (| prop | prop . get (key)) . map (AsRef :: as_ref) } }
};
}
