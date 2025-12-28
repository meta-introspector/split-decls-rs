macro_rules! RgbColor {
    () => {
        # [doc = " 24-bit ANSI RGB color codes"] # [allow (clippy :: exhaustive_structs)] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct RgbColor (pub u8 , pub u8 , pub u8) ;
    };
}

RgbColor!();