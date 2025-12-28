macro_rules! deps {
    () => {
        Mode!();
        Check!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl Mode for Check { type Output < T > = () ; # [inline (always)] fn bind < T , F : FnOnce () -> T > (_ : F) -> Self :: Output < T > { } # [inline (always)] fn map < T , U , F : FnOnce (T) -> U > (_ : Self :: Output < T > , _ : F) -> Self :: Output < U > { } # [inline (always)] fn combine < T , U , V , F : FnOnce (T , U) -> V > (_ : Self :: Output < T > , _ : Self :: Output < U > , _ : F ,) -> Self :: Output < V > { } }
    };
}

impl_162!()