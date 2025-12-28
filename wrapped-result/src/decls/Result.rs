macro_rules! Result {
    () => {
        # [doc = " A specialized [`Result`] type that provides Windows error information."] pub type Result < T > = core :: result :: Result < T , Error > ;
    };
}

Result!()