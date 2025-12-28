macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! yield_now {
    () => {
        deps!();
        # [doc = " A function to help \"yield\" a future, such that it is re-scheduled immediately."] # [doc = ""] # [doc = " Useful for spin counts, so a future doesn't hog too much time."] pub (crate) fn yield_now (cx : & mut Context < '_ >) -> Poll < std :: convert :: Infallible > { cx . waker () . wake_by_ref () ; Poll :: Pending }
    };
}

yield_now!();