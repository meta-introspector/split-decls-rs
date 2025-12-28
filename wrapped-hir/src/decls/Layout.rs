macro_rules! Layout {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq)] pub struct Layout (Arc < TyLayout > , Arc < TargetDataLayout >) ;
    };
}

Layout!();