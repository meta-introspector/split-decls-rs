macro_rules! padding {
    () => {
        # [doc = " Padding size that gives the needed bytes to pad `input` to an integral"] # [doc = " multiple of 16."] fn padding (input : usize) -> usize { if input == 0 { return 0 ; } let rem = input % 16 ; if rem != 0 { 16 - rem } else { 0 } }
    };
}

padding!();