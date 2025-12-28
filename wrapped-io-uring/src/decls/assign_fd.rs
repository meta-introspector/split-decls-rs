macro_rules! deps {
    () => {
        Fd!();
        Fixed!();
    };
}

macro_rules! assign_fd {
    () => {
        deps!();
        macro_rules ! assign_fd { ($ sqe : ident . fd = $ opfd : expr) => { match $ opfd { sealed :: Target :: Fd (fd) => $ sqe . fd = fd , sealed :: Target :: Fixed (idx) => { $ sqe . fd = idx as _ ; $ sqe . flags |= crate :: squeue :: Flags :: FIXED_FILE . bits () ; } } } ; }
    };
}

assign_fd!();