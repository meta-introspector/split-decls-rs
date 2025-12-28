macro_rules! deps {
    () => {
        Either!();
        DisplayValue!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [doc = " Either a static label or a dynamic one implementing [`DisplayValue`]."] # [derive (Clone)] pub enum Kind { # [doc = " Display only the given statically known label."] Label (& 'static str) , # [doc = " Display a label created dynamically."] Dynamic (Arc < dyn DisplayValue + Send + Sync >) , }
    };
}

Kind!();