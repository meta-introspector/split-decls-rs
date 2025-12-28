macro_rules! mod_floor_64 {
    () => {
        # [inline] fn mod_floor_64 (this : i64 , other : i64) -> i64 { match this % other { r if (r > 0 && other < 0) || (r < 0 && other > 0) => r + other , r => r , } }
    };
}

mod_floor_64!()