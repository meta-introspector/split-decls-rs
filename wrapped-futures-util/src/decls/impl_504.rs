macro_rules! impl_504 {
    () => {
        impl < St > FusedStream for BufferUnordered < St > where St : Stream , St :: Item : Future , { fn is_terminated (& self) -> bool { self . in_progress_queue . is_terminated () && self . stream . is_terminated () } }
    };
}

impl_504!();