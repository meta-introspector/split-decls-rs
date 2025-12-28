macro_rules! is_seq_cst {
    () => {
        fn is_seq_cst (order : Ordering) -> bool { order == Ordering :: SeqCst }
    };
}

is_seq_cst!();