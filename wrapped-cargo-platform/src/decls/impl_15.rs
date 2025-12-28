macro_rules! deps {
    () => {
        Cfg!();
        CfgExpr!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl CfgExpr { # [doc = " Utility function to check if the key, \"cfg(..)\" matches the `target_cfg`"] pub fn matches_key (key : & str , target_cfg : & [Cfg]) -> bool { if key . starts_with ("cfg(") && key . ends_with (')') { let cfg = & key [4 .. key . len () - 1] ; CfgExpr :: from_str (cfg) . ok () . map (| ce | ce . matches (target_cfg)) . unwrap_or (false) } else { false } } pub fn matches (& self , cfg : & [Cfg]) -> bool { match * self { CfgExpr :: Not (ref e) => ! e . matches (cfg) , CfgExpr :: All (ref e) => e . iter () . all (| e | e . matches (cfg)) , CfgExpr :: Any (ref e) => e . iter () . any (| e | e . matches (cfg)) , CfgExpr :: Value (ref e) => cfg . contains (e) , CfgExpr :: True => true , CfgExpr :: False => false , } } }
    };
}

impl_15!()