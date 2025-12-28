macro_rules! WinconStream {
    () => {
        # [doc = " Extend `std::io::Write` with wincon styling"] pub trait WinconStream { # [doc = " Write colored text to the stream"] fn write_colored (& mut self , fg : Option < anstyle :: AnsiColor > , bg : Option < anstyle :: AnsiColor > , data : & [u8] ,) -> std :: io :: Result < usize > ; }
    };
}

WinconStream!()