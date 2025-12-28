macro_rules! deps {
    () => {
        Fd!();
        Fixed!();
    };
}

macro_rules! CancelBuilder {
    () => {
        deps!();
        # [doc = " [CancelBuilder] constructs match criteria for request cancellation."] # [doc = ""] # [doc = " The [CancelBuilder] can be used to selectively cancel one or more requests"] # [doc = " by user_data, fd, fixed fd, or unconditionally."] # [doc = ""] # [doc = " ### Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use io_uring::types::{CancelBuilder, Fd, Fixed};"] # [doc = ""] # [doc = " // Match all in-flight requests."] # [doc = " CancelBuilder::any();"] # [doc = ""] # [doc = " // Match a single request with user_data = 42."] # [doc = " CancelBuilder::user_data(42);"] # [doc = ""] # [doc = " // Match a single request with fd = 42."] # [doc = " CancelBuilder::fd(Fd(42));"] # [doc = ""] # [doc = " // Match a single request with fixed fd = 42."] # [doc = " CancelBuilder::fd(Fixed(42));"] # [doc = ""] # [doc = " // Match all in-flight requests with user_data = 42."] # [doc = " CancelBuilder::user_data(42).all();"] # [doc = " ```"] # [derive (Debug)] pub struct CancelBuilder { pub (crate) flags : AsyncCancelFlags , pub (crate) user_data : Option < u64 > , pub (crate) fd : Option < sealed :: Target > , }
    };
}

CancelBuilder!();