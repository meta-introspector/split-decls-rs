macro_rules! deps {
    () => {
        PubNamesEntryIter!();
        Reader!();
        EndianSlice!();
        DebugPubNames!();
        LittleEndian!();
    };
}

macro_rules! impl_532 {
    () => {
        deps!();
        impl < R : Reader > DebugPubNames < R > { # [doc = " Iterate the pubnames in the `.debug_pubnames` section."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugPubNames, EndianSlice, LittleEndian};"] # [doc = ""] # [doc = " # let buf = [];"] # [doc = " # let read_debug_pubnames_section_somehow = || &buf;"] # [doc = " let debug_pubnames ="] # [doc = "     DebugPubNames::new(read_debug_pubnames_section_somehow(), LittleEndian);"] # [doc = ""] # [doc = " let mut iter = debug_pubnames.items();"] # [doc = " while let Some(pubname) = iter.next().unwrap() {"] # [doc = "   println!(\"pubname {} found!\", pubname.name().to_string_lossy());"] # [doc = " }"] # [doc = " ```"] pub fn items (& self) -> PubNamesEntryIter < R > { PubNamesEntryIter (self . 0 . items ()) } }
    };
}

impl_532!()