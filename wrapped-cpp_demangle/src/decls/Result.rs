macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " A demangling result of `T` or a `cpp_demangle::error::Error`."] pub type Result < T > = :: core :: result :: Result < T , Error > ;
    };
}

Result!()