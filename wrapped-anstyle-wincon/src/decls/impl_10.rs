macro_rules! deps {
    () => {
        WinconStream!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl WinconStream for std :: io :: Stdout { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { self . lock () . write_colored (fg , bg , data) } }
    };
}

impl_10!()