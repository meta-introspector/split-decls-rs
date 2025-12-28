macro_rules! Either {
    () => {
        # [doc = " A simple binary sum type."] # [doc = ""] # [doc = " This is occasionally useful in an ad hoc fashion."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum Either < Left , Right > { Left (Left) , Right (Right) , }
    };
}

Either!();