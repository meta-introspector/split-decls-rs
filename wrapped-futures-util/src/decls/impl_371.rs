macro_rules! impl_371 {
    () => {
        impl < S : Stream > FusedStream for Fuse < S > { fn is_terminated (& self) -> bool { self . done } }
    };
}

impl_371!()