macro_rules! deps {
    () => {
        LoadExtensionGuard!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        # [expect (unused_must_use)] impl Drop for LoadExtensionGuard < '_ > { # [inline] fn drop (& mut self) { self . conn . load_extension_disable () ; } }
    };
}

impl_159!();