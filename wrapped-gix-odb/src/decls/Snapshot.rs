macro_rules! deps {
    () => {
        Store!();
        IndexLookup!();
        SlotIndexMarker!();
    };
}

macro_rules! Snapshot {
    () => {
        deps!();
        pub (crate) struct Snapshot { # [doc = " Indices ready for object lookup or contains checks, ordered usually by modification data, recent ones first."] pub (crate) indices : Vec < handle :: IndexLookup > , # [doc = " A set of loose objects dbs to search once packed objects weren't found."] pub (crate) loose_dbs : Arc < Vec < crate :: loose :: Store > > , # [doc = " remember what this state represents and to compare to other states."] pub (crate) marker : types :: SlotIndexMarker , }
    };
}

Snapshot!();