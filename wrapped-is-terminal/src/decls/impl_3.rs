macro_rules! deps {
    () => {
        IsTerminal!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        # [cfg (windows)] impl < Stream : AsHandle > IsTerminal for Stream { # [inline] fn is_terminal (& self) -> bool { handle_is_console (self . as_handle ()) } }
    };
}

impl_3!()