macro_rules! div_rem_64 {
    () => {
        # [inline] fn div_rem_64 (this : i64 , other : i64) -> (i64 , i64) { (this / other , this % other) }
    };
}

div_rem_64!()