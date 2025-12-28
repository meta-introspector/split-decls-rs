macro_rules! deps {
    () => {
        StateID!();
    };
}

macro_rules! u32s_to_state_ids_mut {
    () => {
        deps!();
        # [doc = " Safely converts a `&mut [u32]` to `&mut [StateID]` with zero cost."] pub (crate) fn u32s_to_state_ids_mut (slice : & mut [u32]) -> & mut [StateID] { unsafe { core :: slice :: from_raw_parts_mut (slice . as_mut_ptr () . cast :: < StateID > () , slice . len () ,) } }
    };
}

u32s_to_state_ids_mut!()