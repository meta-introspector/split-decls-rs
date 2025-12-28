macro_rules! XDG_OPEN_SCRIPT {
    () => {
        const XDG_OPEN_SCRIPT : & [u8] = include_bytes ! ("xdg-open") ;
    };
}

XDG_OPEN_SCRIPT!();