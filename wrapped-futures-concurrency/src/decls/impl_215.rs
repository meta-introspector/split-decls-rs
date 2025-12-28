macro_rules! deps {
    () => {
        Join!();
        FutureExt!();
        Race!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl < F1 > FutureExt for F1 where F1 : Future , { fn join < F2 > (self , other : F2) -> Join2 < Self , F2 :: IntoFuture > where Self : Future + Sized , F2 : IntoFuture , { Join :: join ((self , other)) } fn race < T , S2 > (self , other : S2) -> Race2 < T , Self , S2 :: IntoFuture > where Self : Future < Output = T > + Sized , S2 : IntoFuture < Output = T > , { Race :: race ((self , other)) } }
    };
}

impl_215!();