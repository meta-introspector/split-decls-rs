macro_rules! impl_310 {
    () => {
        impl < St > FusedStream for Cycle < St > where St : Clone + Stream , { fn is_terminated (& self) -> bool { matches ! (self . size_hint () , (0 , Some (0))) } }
    };
}

impl_310!();