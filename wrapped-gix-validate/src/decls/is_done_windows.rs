macro_rules! is_done_windows {
    () => {
        # [doc = " Check if trailing filename bytes leave a match to Windows reserved device names unchanged."] fn is_done_windows (input : Option < & [u8] >) -> bool { let Some (input) = input else { return true } ; let skip = input . bytes () . take_while (| b | * b == b' ') . count () ; let Some (next) = input . get (skip) else { return true } ; * next == b'.' || * next == b':' }
    };
}

is_done_windows!();