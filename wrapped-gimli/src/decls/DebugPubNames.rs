macro_rules! deps {
    () => {
        PubNamesEntry!();
        Reader!();
        PubStuffParser!();
        DebugLookup!();
    };
}

macro_rules! DebugPubNames {
    () => {
        deps!();
        # [doc = " The `DebugPubNames` struct represents the DWARF public names information"] # [doc = " found in the `.debug_pubnames` section."] # [derive (Debug , Clone)] pub struct DebugPubNames < R : Reader > (DebugLookup < R , PubStuffParser < R , PubNamesEntry < R > > >) ;
    };
}

DebugPubNames!();