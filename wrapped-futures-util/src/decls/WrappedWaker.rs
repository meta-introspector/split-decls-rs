macro_rules! deps {
    () => {
        SharedPollState!();
    };
}

macro_rules! WrappedWaker {
    () => {
        deps!();
        # [doc = " Will update state with the provided value on `wake_by_ref` call"] # [doc = " and then, if there is a need, call `inner_waker`."] struct WrappedWaker { inner_waker : UnsafeCell < Option < Waker > > , poll_state : SharedPollState , need_to_poll : u8 , }
    };
}

WrappedWaker!();