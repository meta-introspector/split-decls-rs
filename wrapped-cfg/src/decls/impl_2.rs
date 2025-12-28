macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Ord for CfgAtom { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { match (self , other) { (CfgAtom :: Flag (a) , CfgAtom :: Flag (b)) => a . as_str () . cmp (b . as_str ()) , (CfgAtom :: Flag (_) , CfgAtom :: KeyValue { .. }) => std :: cmp :: Ordering :: Less , (CfgAtom :: KeyValue { .. } , CfgAtom :: Flag (_)) => std :: cmp :: Ordering :: Greater , (CfgAtom :: KeyValue { key , value } , CfgAtom :: KeyValue { key : key2 , value : value2 }) => { key . as_str () . cmp (key2 . as_str ()) . then (value . as_str () . cmp (value2 . as_str ())) } } } }
    };
}

impl_2!();