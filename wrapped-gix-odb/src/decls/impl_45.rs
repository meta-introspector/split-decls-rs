macro_rules! deps {
    () => {
        SlotIndexMarker!();
        StateId!();
        Ordering!();
        SlotMapIndex!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl SlotMapIndex { pub (crate) fn state_id (self : & Arc < SlotMapIndex >) -> StateId { let hash = hash :: crc32 (& (Arc :: as_ptr (self) as usize) . to_be_bytes ()) ; hash :: crc32_update (hash , & self . loaded_indices . load (Ordering :: SeqCst) . to_be_bytes ()) } pub (crate) fn marker (self : & Arc < SlotMapIndex >) -> SlotIndexMarker { SlotIndexMarker { generation : self . generation , state_id : self . state_id () , } } # [doc = " Returns true if we already know at least one loose object db, a sign of being initialized"] pub (crate) fn is_initialized (& self) -> bool { ! self . loose_dbs . is_empty () } }
    };
}

impl_45!()