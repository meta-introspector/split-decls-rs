macro_rules! deps {
    () => {
        EntryData!();
        SectionId!();
        Section!();
    };
}

macro_rules! MultiValueMut {
    () => {
        deps!();
        # [doc = " An intermediate representation of a mutable multivar obtained from a [`File`][crate::File]."] # [derive (PartialEq , Eq , Debug)] pub struct MultiValueMut < 'borrow , 'lookup , 'event > { pub (crate) section : & 'borrow mut HashMap < SectionId , Section < 'event > > , pub (crate) key : section :: ValueName < 'lookup > , # [doc = " Each entry data struct provides sufficient information to index into"] # [doc = " [`Self::offsets`]. This layer of indirection is used for users to index"] # [doc = " into the offsets rather than leaking the internal data structures."] pub (crate) indices_and_sizes : Vec < EntryData > , # [doc = " Each offset represents the size of a event slice and whether or not the"] # [doc = " event slice is significant or not. This is used to index into the"] # [doc = " actual section."] pub (crate) offsets : HashMap < SectionId , Vec < usize > > , }
    };
}

MultiValueMut!()