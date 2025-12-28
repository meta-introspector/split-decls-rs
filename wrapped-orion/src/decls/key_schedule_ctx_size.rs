macro_rules! key_schedule_ctx_size {
    () => {
        const fn key_schedule_ctx_size < const NK : usize > () -> usize { (NK * 2) + 1 }
    };
}

key_schedule_ctx_size!()