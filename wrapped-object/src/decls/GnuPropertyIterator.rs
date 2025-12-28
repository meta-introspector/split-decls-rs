macro_rules! deps {
    () => {
        Bytes!();
        Endian!();
    };
}

macro_rules! GnuPropertyIterator {
    () => {
        deps!();
        # [doc = " An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note."] # [doc = ""] # [doc = " Returned by [`Note::gnu_properties`]."] # [derive (Debug)] pub struct GnuPropertyIterator < 'data , Endian : endian :: Endian > { endian : Endian , align : usize , data : Bytes < 'data > , }
    };
}

GnuPropertyIterator!();