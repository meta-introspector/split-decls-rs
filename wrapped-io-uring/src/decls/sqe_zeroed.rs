macro_rules! sqe_zeroed {
    () => {
        # [doc = " inline zeroed to improve codegen"] # [inline (always)] fn sqe_zeroed () -> sys :: io_uring_sqe { unsafe { mem :: zeroed () } }
    };
}

sqe_zeroed!();