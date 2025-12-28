macro_rules! StrLitKind {
    () => {
        # [derive (Clone , Copy)] enum StrLitKind { Normal , Raw (usize) , }
    };
}

StrLitKind!();