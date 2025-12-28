macro_rules! res_zeroed {
    () => {
        # [doc = " inline zeroed to improve codegen"] # [inline (always)] fn res_zeroed () -> sys :: io_uring_restriction { unsafe { std :: mem :: zeroed () } }
    };
}

res_zeroed!();