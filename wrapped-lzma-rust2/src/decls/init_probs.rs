macro_rules! init_probs {
    () => {
        # [inline (always)] pub (crate) fn init_probs (probs : & mut [u16]) { probs . fill (PROB_INIT) ; }
    };
}

init_probs!();