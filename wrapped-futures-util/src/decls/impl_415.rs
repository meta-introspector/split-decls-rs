macro_rules! deps {
    () => {
        FnOnce1!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        impl < St , F > FusedFuture for NextIf < '_ , St , F > where St : Stream , F : for < 'a > FnOnce1 < & 'a St :: Item , Output = bool > , { fn is_terminated (& self) -> bool { self . inner . is_none () } }
    };
}

impl_415!();