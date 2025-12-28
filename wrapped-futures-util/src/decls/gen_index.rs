macro_rules! gen_index {
    () => {
        # [doc = " Return a value from `0..n`."] fn gen_index (n : usize) -> usize { (random () % n as u64) as usize }
    };
}

gen_index!();