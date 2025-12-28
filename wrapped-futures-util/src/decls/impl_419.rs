macro_rules! impl_419 {
    () => {
        impl < St , T > FusedFuture for NextIfEq < '_ , St , T > where St : Stream , T : ? Sized , St :: Item : PartialEq < T > , { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
    };
}

impl_419!()