macro_rules! Label {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct Label { pub name : Name , }
    };
}

Label!();