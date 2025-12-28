macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! fixup_windows_path {
    () => {
        deps!();
        # [cfg (not (windows))] fn fixup_windows_path (path : CString) -> Result < CString , Error > { Ok (path) }
    };
}

fixup_windows_path!();