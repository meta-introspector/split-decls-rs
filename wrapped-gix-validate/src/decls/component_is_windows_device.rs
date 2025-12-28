macro_rules! component_is_windows_device {
    () => {
        # [doc = " Return `true` if the path component at `input` looks like a Windows device, like `CON`"] # [doc = " or `LPT1` (case-insensitively)."] # [doc = ""] # [doc = " This is relevant only on Windows, where one may be tricked into reading or writing to such devices."] # [doc = " When reading from `CON`, a console-program may block until the user provided input."] pub fn component_is_windows_device (input : & BStr) -> bool { is_win_device (input) }
    };
}

component_is_windows_device!();