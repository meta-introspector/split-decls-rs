macro_rules! deps {
    () => {
        Result!();
        Context!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [cfg (not (feature = "anyhow_enabled"))] impl < T , E > Context < T > for std :: result :: Result < T , E > where E : std :: error :: Error + Send + Sync + 'static , { fn context < C > (self , _context : C) -> Result < T > where C : std :: fmt :: Display + Send + Sync + 'static , { self . map_err (| e | Box :: new (e) as Box < dyn Error >) } }
    };
}

impl_49!()