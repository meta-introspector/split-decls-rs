macro_rules! deps {
    () => {
        LocalVars!();
    };
}

macro_rules! undo_bytes {
    () => {
        deps!();
        # [inline] fn undo_bytes (l : & mut LocalVars , max : u32) -> u32 { let res = cmp :: min (l . num_bits >> 3 , max) ; l . num_bits -= res << 3 ; res }
    };
}

undo_bytes!();