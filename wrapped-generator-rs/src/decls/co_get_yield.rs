macro_rules! deps {
    () => {
        ContextStack!();
    };
}

macro_rules! co_get_yield {
    () => {
        deps!();
        # [doc = " coroutine get passed in yield para"] pub fn co_get_yield < A : Any > () -> Option < A > { ContextStack :: current () . co_ctx () . and_then (| ctx | ctx . co_get_para ()) }
    };
}

co_get_yield!();