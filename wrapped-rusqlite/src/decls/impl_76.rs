macro_rules! deps {
    () => {
        Column!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [cfg (feature = "column_decltype")] impl Column < '_ > { # [doc = " Returns the name of the column."] # [inline] # [must_use] pub fn name (& self) -> & str { self . name } # [doc = " Returns the type of the column (`None` for expression)."] # [inline] # [must_use] pub fn decl_type (& self) -> Option < & str > { self . decl_type } }
    };
}

impl_76!()