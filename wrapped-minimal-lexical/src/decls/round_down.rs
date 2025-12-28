macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! round_down {
    () => {
        deps!();
        # [doc = " Round our significant digits into place, truncating them."] # [cfg_attr (not (feature = "compact") , inline)] pub fn round_down (fp : & mut ExtendedFloat , shift : i32) { fp . mant = match shift == 64 { true => 0 , false => fp . mant >> shift , } ; fp . exp += shift ; }
    };
}

round_down!()