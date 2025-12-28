macro_rules! InterruptDrawInfo {
    () => {
        # [derive (Clone , Copy)] pub (crate) enum InterruptDrawInfo { Instantly , # [doc = " Boolean signals if interrupt is requested"] Deferred (bool) , }
    };
}

InterruptDrawInfo!();