macro_rules! div_mod_floor_64 {
    () => {
        # [inline] fn div_mod_floor_64 (this : i64 , other : i64) -> (i64 , i64) { (div_floor_64 (this , other) , mod_floor_64 (this , other)) }
    };
}

div_mod_floor_64!()