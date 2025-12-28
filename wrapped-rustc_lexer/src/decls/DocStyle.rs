macro_rules! DocStyle {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DocStyle { Outer , Inner , }
    };
}

DocStyle!();