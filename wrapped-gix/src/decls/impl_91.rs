macro_rules! deps {
    () => {
        PrepareFetch!();
        Repository!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        # [doc = " Consumption"] impl PrepareFetch { # [doc = " Persist the contained repository as is even if an error may have occurred when fetching from the remote."] pub fn persist (mut self) -> Repository { self . repo . take () . expect ("present and consumed once") } }
    };
}

impl_91!()