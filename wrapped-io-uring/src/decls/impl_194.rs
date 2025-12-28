macro_rules! deps {
    () => {
        DestinationSlot!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl DestinationSlot { const AUTO_ALLOC : NonZeroU32 = unwrap_nonzero (NonZeroU32 :: new (sys :: IORING_FILE_INDEX_ALLOC as u32)) ; # [doc = " Use an automatically allocated target slot."] pub const fn auto_target () -> Self { Self { dest : DestinationSlot :: AUTO_ALLOC , } } # [doc = " Try to use a given target slot."] # [doc = ""] # [doc = " Valid slots are in the range from `0` to `u32::MAX - 2` inclusive."] pub fn try_from_slot_target (target : u32) -> Result < Self , u32 > { const MAX_INDEX : u32 = unwrap_u32 (DestinationSlot :: AUTO_ALLOC . get () . checked_sub (2)) ; if target > MAX_INDEX { return Err (target) ; } let kernel_index = target . saturating_add (1) ; debug_assert ! (0 < kernel_index && kernel_index < DestinationSlot :: AUTO_ALLOC . get ()) ; let dest = NonZeroU32 :: new (kernel_index) . unwrap () ; Ok (Self { dest }) } pub (crate) fn kernel_index_arg (& self) -> u32 { self . dest . get () } }
    };
}

impl_194!()