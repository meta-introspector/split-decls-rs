macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! UnameField {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [allow (dead_code)] pub enum UnameField { Sysname , Release , Version , Machine , Nodename , OperatingSystem , }
    };
}

UnameField!()