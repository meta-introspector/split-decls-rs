macro_rules! DropCounter {
    () => {
        struct DropCounter < 'a > { count : & 'a Cell < u32 > , }
    };
}

DropCounter!();