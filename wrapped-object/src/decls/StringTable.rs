macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! StringTable {
    () => {
        deps!();
        # [derive (Debug , Default)] pub (crate) struct StringTable < 'a > { strings : IndexSet < & 'a [u8] > , offsets : Vec < usize > , }
    };
}

StringTable!()