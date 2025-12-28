macro_rules! deps {
    () => {
        PubTypesEntry!();
        PubStuffParser!();
        DebugLookup!();
        Reader!();
    };
}

macro_rules! DebugPubTypes {
    () => {
        deps!();
        # [doc = " The `DebugPubTypes` struct represents the DWARF public types information"] # [doc = " found in the `.debug_info` section."] # [derive (Debug , Clone)] pub struct DebugPubTypes < R : Reader > (DebugLookup < R , PubStuffParser < R , PubTypesEntry < R > > >) ;
    };
}

DebugPubTypes!()