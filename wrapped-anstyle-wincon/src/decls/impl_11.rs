macro_rules! deps {
    () => {
        WinconStream!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl WinconStream for std :: io :: Stderr { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { self . lock () . write_colored (fg , bg , data) } }
    };
}

impl_11!()