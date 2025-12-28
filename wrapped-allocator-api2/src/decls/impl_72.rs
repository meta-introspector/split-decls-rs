macro_rules! deps {
    () => {
        Box!();
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < B : std :: io :: BufRead + ? Sized , A : Allocator > std :: io :: BufRead for Box < B , A > { # [inline] fn fill_buf (& mut self) -> std :: io :: Result < & [u8] > { (* * self) . fill_buf () } # [inline] fn consume (& mut self , amt : usize) { (* * self) . consume (amt) } # [inline] fn read_until (& mut self , byte : u8 , buf : & mut std :: vec :: Vec < u8 >) -> std :: io :: Result < usize > { (* * self) . read_until (byte , buf) } # [inline] fn read_line (& mut self , buf : & mut std :: string :: String) -> std :: io :: Result < usize > { (* * self) . read_line (buf) } }
    };
}

impl_72!()