macro_rules! Platform {
    () => {
        # [doc = " Platform definition."] # [derive (Eq , PartialEq , Hash , Ord , PartialOrd , Clone , Debug)] pub enum Platform { # [doc = " A named platform, like `x86_64-apple-darwin`."] Name (String) , # [doc = " A cfg expression, like `cfg(windows)`."] Cfg (CfgExpr) , }
    };
}

Platform!()