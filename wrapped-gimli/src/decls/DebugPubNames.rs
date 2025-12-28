macro_rules! deps {
    () => {
        Reader!();
        DebugLookup!();
        PubNamesEntry!();
        PubStuffParser!();
    };
}

macro_rules! DebugPubNames {
    () => {
        deps!();
        # [doc = " The `DebugPubNames` struct represents the DWARF public names information"] # [doc = " found in the `.debug_pubnames` section."] # [derive (Debug , Clone)] pub struct DebugPubNames < R : Reader > (DebugLookup < R , PubStuffParser < R , PubNamesEntry < R > > >) ;
    };
}

DebugPubNames!()