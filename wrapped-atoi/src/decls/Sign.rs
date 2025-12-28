macro_rules! Sign {
    () => {
        # [doc = " Representation of a numerical sign"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Sign { Plus , Minus , }
    };
}

Sign!()