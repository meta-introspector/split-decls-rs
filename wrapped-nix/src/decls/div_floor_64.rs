macro_rules! div_floor_64 {
    () => {
        # [inline] fn div_floor_64 (this : i64 , other : i64) -> i64 { match div_rem_64 (this , other) { (d , r) if (r > 0 && other < 0) || (r < 0 && other > 0) => d - 1 , (d , _) => d , } }
    };
}

div_floor_64!()