macro_rules! CargoFeatures {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum CargoFeatures { All , Selected { # [doc = " List of features to activate."] features : Vec < String > , # [doc = " Do not activate the `default` feature."] no_default_features : bool , } , }
    };
}

CargoFeatures!();