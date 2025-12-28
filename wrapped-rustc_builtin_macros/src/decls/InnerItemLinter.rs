macro_rules! InnerItemLinter {
    () => {
        struct InnerItemLinter < 'a > { sess : & 'a Session , }
    };
}

InnerItemLinter!();