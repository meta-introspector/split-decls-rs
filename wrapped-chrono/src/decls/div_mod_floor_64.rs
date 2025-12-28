macro_rules! div_mod_floor_64 {
    () => {
        # [inline] const fn div_mod_floor_64 (this : i64 , other : i64) -> (i64 , i64) { (this . div_euclid (other) , this . rem_euclid (other)) }
    };
}

div_mod_floor_64!()