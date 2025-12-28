macro_rules! deps {
    () => {
        FileRuntime!();
    };
}

macro_rules! Runtime {
    () => {
        deps!();
        # [derive (Default)] struct Runtime { help_printed : bool , per_file : HashMap < & 'static str , FileRuntime > , }
    };
}

Runtime!();