macro_rules! deps {
    () => {
        WinconStream!();
        Buffer!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        # [cfg (all (windows , feature = "wincon"))] impl anstyle_wincon :: WinconStream for Buffer { fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > { self . 0 . write_colored (fg , bg , data) } }
    };
}

impl_106!()