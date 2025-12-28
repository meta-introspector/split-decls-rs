macro_rules! deps {
    () => {
        YieldNow!();
    };
}

macro_rules! yield_now {
    () => {
        deps!();
        # [doc = " Wakes the current task and returns [`Poll::Pending`] once."] # [doc = ""] # [doc = " This function is useful when we want to cooperatively give time to the task scheduler. It is"] # [doc = " generally a good idea to yield inside loops because that way we make sure long-running tasks"] # [doc = " don't prevent other tasks from running."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " future::yield_now().await;"] # [doc = " # })"] # [doc = " ```"] pub fn yield_now () -> YieldNow { YieldNow (false) }
    };
}

yield_now!()