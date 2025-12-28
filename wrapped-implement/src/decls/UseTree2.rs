macro_rules! deps {
    () => {
        UsePath2!();
        UseName2!();
        UseGroup2!();
    };
}

macro_rules! UseTree2 {
    () => {
        deps!();
        enum UseTree2 { Path (UsePath2) , Name (UseName2) , Group (UseGroup2) , TrustLevel (usize) , Agile (bool) , }
    };
}

UseTree2!()