macro_rules! q_of_o {
    () => {
        # [inline (always)] fn q_of_o (o : u128) -> [u64 ; 2] { [o as u64 , (o >> 64) as u64] }
    };
}

q_of_o!();