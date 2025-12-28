macro_rules! div_mod_floor {
    () => {
        const fn div_mod_floor (val : i32 , div : i32) -> (i32 , i32) { (val . div_euclid (div) , val . rem_euclid (div)) }
    };
}

div_mod_floor!()