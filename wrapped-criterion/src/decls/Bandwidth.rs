macro_rules! Bandwidth {
    () => {
        # [doc = " Method to estimate the bandwidth"] pub enum Bandwidth { # [doc = " Use Silverman's rule of thumb to estimate the bandwidth from the sample"] Silverman , }
    };
}

Bandwidth!()