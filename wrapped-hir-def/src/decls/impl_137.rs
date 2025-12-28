macro_rules! deps {
    () => {
        KeyMap!();
        Policy!();
        Key!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < P : Policy > KeyMap < Key < P :: K , P :: V , P > > { pub fn insert (& mut self , key : P :: K , value : P :: V) { P :: insert (& mut self . map , key , value) } pub fn get (& self , key : & P :: K) -> Option < & P :: V > { P :: get (& self . map , key) } pub fn is_empty (& self) -> bool { P :: is_empty (& self . map) } }
    };
}

impl_137!()