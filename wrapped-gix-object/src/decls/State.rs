macro_rules! State {
    () => {
        # [derive (Default , Copy , Clone)] pub (crate) enum State { # [default] Target , TargetKind , Name , Tagger , Message , }
    };
}

State!()