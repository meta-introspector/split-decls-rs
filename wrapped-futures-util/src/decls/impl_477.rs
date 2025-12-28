macro_rules! impl_477 {
    () => {
        impl < St1 , St2 > FusedStream for Zip < St1 , St2 > where St1 : Stream , St2 : Stream , { fn is_terminated (& self) -> bool { self . stream1 . is_terminated () && self . stream2 . is_terminated () } }
    };
}

impl_477!()