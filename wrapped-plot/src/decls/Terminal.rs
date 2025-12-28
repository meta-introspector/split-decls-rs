macro_rules! deps {
    () => {
        Output!();
    };
}

macro_rules! Terminal {
    () => {
        deps!();
        # [doc = " Output terminal"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum Terminal { Svg , }
    };
}

Terminal!()