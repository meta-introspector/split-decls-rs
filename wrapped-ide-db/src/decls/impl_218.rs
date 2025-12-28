macro_rules! deps {
    () => {
        Indel!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl Indel { pub fn insert (offset : TextSize , text : String) -> Indel { Indel :: replace (TextRange :: empty (offset) , text) } pub fn delete (range : TextRange) -> Indel { Indel :: replace (range , String :: new ()) } pub fn replace (range : TextRange , replace_with : String) -> Indel { Indel { delete : range , insert : replace_with } } pub fn apply (& self , text : & mut String) { let start : usize = self . delete . start () . into () ; let end : usize = self . delete . end () . into () ; text . replace_range (start .. end , & self . insert) ; } }
    };
}

impl_218!();