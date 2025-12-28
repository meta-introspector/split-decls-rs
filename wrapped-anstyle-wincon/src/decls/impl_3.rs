macro_rules! deps {
    () => {
        WinconStream!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T : WinconStream + ? Sized > WinconStream for & mut T { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { (* * self) . write_colored (fg , bg , data) } }
    };
}

impl_3!();