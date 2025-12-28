macro_rules! Result {
    () => {
        # [doc = " Nix Result Type"] pub type Result < T > = result :: Result < T , Errno > ;
    };
}

Result!()