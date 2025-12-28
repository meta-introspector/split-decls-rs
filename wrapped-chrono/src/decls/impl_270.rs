macro_rules! deps {
    () => {
        Numeric!();
        Item!();
        Fixed!();
        Error!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        # [cfg (feature = "defmt")] impl < 'a > defmt :: Format for Item < 'a > { fn format (& self , f : defmt :: Formatter) { match self { Item :: Literal (v) => defmt :: write ! (f , "Literal {{ {} }}" , v) , # [cfg (feature = "alloc")] Item :: OwnedLiteral (_) => { } Item :: Space (v) => defmt :: write ! (f , "Space {{ {}  }}" , v) , # [cfg (feature = "alloc")] Item :: OwnedSpace (_) => { } Item :: Numeric (u , v) => defmt :: write ! (f , "Numeric {{ {}, {} }}" , u , v) , Item :: Fixed (v) => defmt :: write ! (f , "Fixed {{ {}  }}" , v) , Item :: Error => defmt :: write ! (f , "Error") , } } }
    };
}

impl_270!();