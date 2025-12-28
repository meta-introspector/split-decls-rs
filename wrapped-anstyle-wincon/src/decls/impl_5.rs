macro_rules! deps {
    () => {
        WinconStream!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl WinconStream for dyn std :: io :: Write { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { crate :: ansi :: write_colored (self , fg , bg , data) } }
    };
}

impl_5!();