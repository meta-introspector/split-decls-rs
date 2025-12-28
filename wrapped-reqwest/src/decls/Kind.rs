macro_rules! Kind {
    () => {
        # [derive (Debug)] pub (crate) enum Kind { Builder , Request , Redirect , # [cfg (not (target_arch = "wasm32"))] Status (StatusCode , Option < hyper :: ext :: ReasonPhrase >) , # [cfg (target_arch = "wasm32")] Status (StatusCode) , Body , Decode , Upgrade , }
    };
}

Kind!()