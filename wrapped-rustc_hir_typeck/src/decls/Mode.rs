macro_rules! Mode {
    () => {
        # [derive (PartialEq , Eq , Copy , Clone , Debug)] pub (crate) enum Mode { MethodCall , Path , }
    };
}

Mode!()