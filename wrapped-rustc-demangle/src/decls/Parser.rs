macro_rules! Parser {
    () => {
        struct Parser < 's > { sym : & 's str , next : usize , depth : u32 , }
    };
}

Parser!();