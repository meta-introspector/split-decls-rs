macro_rules! StdError {
    () => {
        # [cfg (all (not (feature = "std") , anyhow_no_core_error))] trait StdError : Debug + Display { fn source (& self) -> Option < & (dyn StdError + 'static) > { None } }
    };
}

StdError!()