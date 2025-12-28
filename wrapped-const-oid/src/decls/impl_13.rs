macro_rules! deps {
    () => {
        Result!();
        Arc!();
        RootArcs!();
        Error!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl RootArcs { # [doc = " Create [`RootArcs`] from the first and second arc values represented"] # [doc = " as `Arc` integers."] pub (crate) const fn new (first_arc : Arc , second_arc : Arc) -> Result < Self > { if first_arc > ARC_MAX_FIRST { return Err (Error :: ArcInvalid { arc : first_arc }) ; } if second_arc > ARC_MAX_SECOND { return Err (Error :: ArcInvalid { arc : second_arc }) ; } # [allow (clippy :: arithmetic_side_effects)] let byte = (first_arc * (ARC_MAX_SECOND + 1)) as u8 + second_arc as u8 ; Ok (Self (byte)) } # [doc = " Get the value of the first arc"] # [allow (clippy :: arithmetic_side_effects)] pub (crate) const fn first_arc (self) -> Arc { self . 0 as Arc / (ARC_MAX_SECOND + 1) } # [doc = " Get the value of the second arc"] # [allow (clippy :: arithmetic_side_effects)] pub (crate) const fn second_arc (self) -> Arc { self . 0 as Arc % (ARC_MAX_SECOND + 1) } }
    };
}

impl_13!();