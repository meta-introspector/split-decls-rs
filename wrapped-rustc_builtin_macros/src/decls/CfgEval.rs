macro_rules! CfgEval {
    () => {
        struct CfgEval < 'a > (StripUnconfigured < 'a >) ;
    };
}

CfgEval!()