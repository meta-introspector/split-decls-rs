macro_rules! ClosingBrackets {
    () => {
        struct ClosingBrackets { span : Span , count : usize , empty_alt : bool , }
    };
}

ClosingBrackets!();