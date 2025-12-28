macro_rules! Status {
    () => {
        # [doc = " The return status of an [invoked command][Client::invoke()]."] # [derive (Debug , Clone)] pub enum Status { # [doc = " No new status was set, and nothing was sent, so instead we are to assume the previous status is still in effect."] Previous , # [doc = " Something was sent, but we couldn't identify it as status."] Unset , # [doc = " Assume the given named status."] Named (String) , }
    };
}

Status!();