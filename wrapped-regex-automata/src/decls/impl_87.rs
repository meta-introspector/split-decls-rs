macro_rules! deps {
    () => {
        LookSet!();
        Slots!();
        Epsilons!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Epsilons { const SLOT_MASK : u64 = 0x000003FF_FFFFFC00 ; const SLOT_SHIFT : u64 = 10 ; const LOOK_MASK : u64 = 0x00000000_000003FF ; # [doc = " Create a new empty epsilons. It has no slots and no assertions that"] # [doc = " need to be satisfied."] fn empty () -> Epsilons { Epsilons (0) } # [doc = " Returns true if this epsilons contains no slots and no assertions."] fn is_empty (self) -> bool { self . 0 == 0 } # [doc = " Returns the slot epsilon transitions."] fn slots (self) -> Slots { Slots ((self . 0 >> Epsilons :: SLOT_SHIFT) . low_u32 ()) } # [doc = " Set the slot epsilon transitions."] fn set_slots (self , slots : Slots) -> Epsilons { Epsilons ((u64 :: from (slots . 0) << Epsilons :: SLOT_SHIFT) | (self . 0 & Epsilons :: LOOK_MASK) ,) } # [doc = " Return the set of look-around assertions in these epsilon transitions."] fn looks (self) -> LookSet { LookSet { bits : (self . 0 & Epsilons :: LOOK_MASK) . low_u32 () } } # [doc = " Set the look-around assertions on these epsilon transitions."] fn set_looks (self , look_set : LookSet) -> Epsilons { Epsilons ((self . 0 & Epsilons :: SLOT_MASK) | (u64 :: from (look_set . bits) & Epsilons :: LOOK_MASK) ,) } }
    };
}

impl_87!()