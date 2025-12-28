macro_rules! full_multiplication {
    () => {
        # [inline] fn full_multiplication (a : u64 , b : u64) -> (u64 , u64) { let r = (a as u128) * (b as u128) ; (r as u64 , (r >> 64) as u64) }
    };
}

full_multiplication!()