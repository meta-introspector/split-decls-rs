macro_rules! deps {
    () => {
        FunctionType!();
        IndexStr!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl FunctionType { # [inline] fn starts_with (input : & IndexStr) -> bool { input . peek () == Some (b'F') || (input . peek () == Some (b'D') && (matches ! (input . peek_second () , Some (b'o') | Some (b'O') | Some (b'x') | Some (b'w')))) } }
    };
}

impl_189!();