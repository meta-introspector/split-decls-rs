macro_rules! deps {
    () => {
        OffsetPrecision!();
        Pad!();
        Colons!();
    };
}

macro_rules! OffsetFormat {
    () => {
        deps!();
        # [doc = " Type for specifying the format of UTC offsets."] # [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct OffsetFormat { # [doc = " See `OffsetPrecision`."] pub precision : OffsetPrecision , # [doc = " Separator between hours, minutes and seconds."] pub colons : Colons , # [doc = " Represent `+00:00` as `Z`."] pub allow_zulu : bool , # [doc = " Pad the hour value to two digits."] pub padding : Pad , }
    };
}

OffsetFormat!();