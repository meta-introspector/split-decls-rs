macro_rules! ONE_INACTIVE {
    () => {
        # [doc = " Constant that can be added to add one inactive thread."] # [doc = " An inactive thread is either idle, sleepy, or sleeping."] const ONE_INACTIVE : usize = 1 << INACTIVE_SHIFT ;
    };
}

ONE_INACTIVE!()