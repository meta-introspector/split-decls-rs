macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < W : std :: io :: Write + ? Sized , A : Allocator > std :: io :: Write for Box < W , A > { # [inline] fn write (& mut self , buf : & [u8]) -> std :: io :: Result < usize > { (* * self) . write (buf) } # [inline] fn flush (& mut self) -> std :: io :: Result < () > { (* * self) . flush () } # [inline] fn write_all (& mut self , buf : & [u8]) -> std :: io :: Result < () > { (* * self) . write_all (buf) } # [inline] fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> std :: io :: Result < () > { (* * self) . write_fmt (fmt) } }
    };
}

impl_70!()