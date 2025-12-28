macro_rules! deps {
    () => {
        RootArcs!();
        Result!();
        Arc!();
        Error!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl TryFrom < u8 > for RootArcs { type Error = Error ; # [allow (clippy :: arithmetic_side_effects)] fn try_from (octet : u8) -> Result < Self > { let first = octet as Arc / (ARC_MAX_SECOND + 1) ; let second = octet as Arc % (ARC_MAX_SECOND + 1) ; let result = Self :: new (first , second) ? ; debug_assert_eq ! (octet , result . 0) ; Ok (result) } }
    };
}

impl_14!();