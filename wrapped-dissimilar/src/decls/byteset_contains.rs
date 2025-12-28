macro_rules! byteset_contains {
    () => {
        fn byteset_contains (byteset : u64 , ch : char) -> bool { (byteset >> ((ch as u8 & 0x3f) as usize)) & 1 != 0 }
    };
}

byteset_contains!()