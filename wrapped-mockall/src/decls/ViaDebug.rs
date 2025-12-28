macro_rules! deps {
    () => {
        DebugPrint!();
    };
}

macro_rules! ViaDebug {
    () => {
        deps!();
        # [doc (hidden)] pub trait ViaDebug < T > where T : Debug { fn debug_string (& self) -> DebugPrint < '_ , T > ; }
    };
}

ViaDebug!();