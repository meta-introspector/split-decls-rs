macro_rules! Rgb {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Rgb { # [doc = " Red"] pub r : u8 , # [doc = " Green"] pub g : u8 , # [doc = " Blue"] pub b : u8 , }
    };
}

Rgb!();