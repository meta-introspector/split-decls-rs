macro_rules! deps {
    () => {
        EncodeError!();
        Writer!();
    };
}

macro_rules! encode_utf8 {
    () => {
        deps!();
        fn encode_utf8 (writer : & mut impl Writer , c : char) -> Result < () , EncodeError > { let code = c as u32 ; if code < MAX_ONE_B { writer . write (& [c as u8]) } else if code < MAX_TWO_B { let mut buf = [0u8 ; 2] ; buf [0] = ((code >> 6) & 0x1F) as u8 | TAG_TWO_B ; buf [1] = (code & 0x3F) as u8 | TAG_CONT ; writer . write (& buf) } else if code < MAX_THREE_B { let mut buf = [0u8 ; 3] ; buf [0] = ((code >> 12) & 0x0F) as u8 | TAG_THREE_B ; buf [1] = ((code >> 6) & 0x3F) as u8 | TAG_CONT ; buf [2] = (code & 0x3F) as u8 | TAG_CONT ; writer . write (& buf) } else { let mut buf = [0u8 ; 4] ; buf [0] = ((code >> 18) & 0x07) as u8 | TAG_FOUR_B ; buf [1] = ((code >> 12) & 0x3F) as u8 | TAG_CONT ; buf [2] = ((code >> 6) & 0x3F) as u8 | TAG_CONT ; buf [3] = (code & 0x3F) as u8 | TAG_CONT ; writer . write (& buf) } }
    };
}

encode_utf8!();