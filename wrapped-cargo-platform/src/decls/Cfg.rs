macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! Cfg {
    () => {
        deps!();
        # [doc = " A cfg value."] # [derive (Eq , PartialEq , Hash , Ord , PartialOrd , Clone , Debug)] pub enum Cfg { # [doc = " A named cfg value, like `unix`."] Name (Ident) , # [doc = " A key/value cfg pair, like `target_os = \"linux\"`."] KeyPair (Ident , String) , }
    };
}

Cfg!()