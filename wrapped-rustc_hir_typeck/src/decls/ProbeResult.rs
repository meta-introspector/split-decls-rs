macro_rules! ProbeResult {
    () => {
        # [derive (Debug , PartialEq , Eq , Copy , Clone)] enum ProbeResult { NoMatch , BadReturnType , Match , }
    };
}

ProbeResult!()