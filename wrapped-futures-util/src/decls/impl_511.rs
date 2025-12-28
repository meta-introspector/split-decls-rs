macro_rules! impl_511 {
    () => {
        impl < St > FusedStream for Buffered < St > where St : Stream , St :: Item : Future , { fn is_terminated (& self) -> bool { self . stream . is_done () && self . in_progress_queue . is_terminated () } }
    };
}

impl_511!()