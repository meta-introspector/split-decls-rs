macro_rules! enable_virtual_terminal_processing {
    () => {
        # [doc = " Raw `ENABLE_VIRTUAL_TERMINAL_PROCESSING` on stdout/stderr"] # [cfg (windows)] pub fn enable_virtual_terminal_processing () -> std :: io :: Result < () > { windows_console :: enable_virtual_terminal_processing () }
    };
}

enable_virtual_terminal_processing!()