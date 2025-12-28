macro_rules! deps {
    () => {
        MACHINE_ATTRIBUTES!();
        HRESULT!();
    };
}

macro_rules! macro_26 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetMachineTypeAttributes (machine : u16 , machinetypeattributes : * mut MACHINE_ATTRIBUTES) -> HRESULT) ;
    };
}

macro_26!();