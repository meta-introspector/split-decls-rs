macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! now_or_never {
    () => {
        deps!();
        # [doc = " Poll the future once and return `Some` if it is ready, else `None`."] # [doc = ""] # [doc = " If the future wasn't ready, it future likely can't be driven to completion any more: the polling"] # [doc = " uses a no-op waker, so knowledge of what the pending future was waiting for is lost."] # [cfg (feature = "client")] pub (crate) fn now_or_never < F : std :: future :: Future > (fut : F) -> Option < F :: Output > { let waker = noop_waker () ; let mut cx = Context :: from_waker (& waker) ; pin_utils :: pin_mut ! (fut) ; match fut . poll (& mut cx) { Poll :: Ready (res) => Some (res) , Poll :: Pending => None , } }
    };
}

now_or_never!()