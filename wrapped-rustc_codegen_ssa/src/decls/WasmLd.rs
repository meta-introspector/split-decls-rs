macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! WasmLd {
    () => {
        deps!();
        struct WasmLd < 'a > { cmd : Command , sess : & 'a Session , }
    };
}

WasmLd!();