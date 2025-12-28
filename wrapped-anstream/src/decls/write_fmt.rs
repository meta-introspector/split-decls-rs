macro_rules! deps {
    () => {
        WinconBytes!();
        Adapter!();
        WinconStream!();
    };
}

macro_rules! write_fmt {
    () => {
        deps!();
        fn write_fmt (raw : & mut dyn anstyle_wincon :: WinconStream , state : & mut WinconBytes , args : std :: fmt :: Arguments < '_ > ,) -> std :: io :: Result < () > { let write_all = | buf : & [u8] | write_all (raw , state , buf) ; crate :: fmt :: Adapter :: new (write_all) . write_fmt (args) }
    };
}

write_fmt!()