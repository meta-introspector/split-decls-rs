macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Short hand for [`Result`] type"] # [doc = ""] # [doc = " [`Result`]: std::result::Result"] pub type Result < T , E = Error > = StdResult < T , E > ;
    };
}

Result!()