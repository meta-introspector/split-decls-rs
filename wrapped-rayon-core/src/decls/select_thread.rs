macro_rules! select_thread {
    () => {
        # [inline] fn select_thread (word : usize , shift : usize) -> usize { (word >> shift) & THREADS_MAX }
    };
}

select_thread!()