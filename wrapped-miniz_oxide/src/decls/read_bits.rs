macro_rules! deps {
    () => {
        InputWrapper!();
        Action!();
        BitBuffer!();
        LocalVars!();
    };
}

macro_rules! read_bits {
    () => {
        deps!();
        # [doc = " Try to read `amount` number of bits from `in_iter` and call the function `f` with the bits as an"] # [doc = " an argument after reading, returning the result of that function, or `Action::End` if there are"] # [doc = " not enough bytes left."] # [inline] # [allow (clippy :: while_immutable_condition)] fn read_bits < F > (l : & mut LocalVars , amount : u32 , in_iter : & mut InputWrapper , flags : u32 , f : F ,) -> Action where F : FnOnce (& mut LocalVars , BitBuffer) -> Action , { while l . num_bits < amount { let action = read_byte (in_iter , flags , | byte | { l . bit_buf |= BitBuffer :: from (byte) << l . num_bits ; l . num_bits += 8 ; Action :: None }) ; if ! matches ! (action , Action :: None) { return action ; } } let bits = l . bit_buf & ((1 << amount) - 1) ; l . bit_buf >>= amount ; l . num_bits -= amount ; f (l , bits) }
    };
}

read_bits!()