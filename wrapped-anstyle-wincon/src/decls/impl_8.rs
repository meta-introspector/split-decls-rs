macro_rules! deps {
    () => {
        WinconStream!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl WinconStream for std :: fs :: File { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { crate :: ansi :: write_colored (self , fg , bg , data) } }
    };
}

impl_8!();