macro_rules! deps {
    () => {
        Whitespace!();
        Section!();
    };
}

macro_rules! SectionMut {
    () => {
        deps!();
        # [doc = " A opaque type that represents a mutable reference to a section."] # [derive (PartialEq , Eq , Hash , PartialOrd , Ord , Debug)] pub struct SectionMut < 'a , 'event > { section : & 'a mut Section < 'event > , implicit_newline : bool , whitespace : Whitespace < 'event > , newline : SmallVec < u8 , 2 > , }
    };
}

SectionMut!();