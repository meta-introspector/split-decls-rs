macro_rules! deps {
    () => {
        Repository!();
        PrepareCheckout!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        # [doc = " Consumption"] impl PrepareCheckout { # [doc = " Persist the contained repository as is even if an error may have occurred when checking out the main working tree."] pub fn persist (mut self) -> Repository { self . repo . take () . expect ("present and consumed once") } }
    };
}

impl_97!()