macro_rules! deps {
    () => {
        HRESULT!();
        MACHINE_ATTRIBUTES!();
    };
}

macro_rules! macro_26 {
    () => {
        deps!();
        windows_link :: link ! ("kernel32.dll" "system" fn GetMachineTypeAttributes (machine : u16 , machinetypeattributes : * mut MACHINE_ATTRIBUTES) -> HRESULT) ;
    };
}

macro_26!()