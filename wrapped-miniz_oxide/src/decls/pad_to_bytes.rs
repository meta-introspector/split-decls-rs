macro_rules! deps {
    () => {
        InputWrapper!();
        Action!();
        LocalVars!();
    };
}

macro_rules! pad_to_bytes {
    () => {
        deps!();
        # [inline] fn pad_to_bytes < F > (l : & mut LocalVars , in_iter : & mut InputWrapper , flags : u32 , f : F) -> Action where F : FnOnce (& mut LocalVars) -> Action , { let num_bits = l . num_bits & 7 ; read_bits (l , num_bits , in_iter , flags , | l , _ | f (l)) }
    };
}

pad_to_bytes!();