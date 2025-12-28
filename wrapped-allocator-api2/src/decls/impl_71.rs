macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < S : std :: io :: Seek + ? Sized , A : Allocator > std :: io :: Seek for Box < S , A > { # [inline] fn seek (& mut self , pos : std :: io :: SeekFrom) -> std :: io :: Result < u64 > { (* * self) . seek (pos) } # [inline] fn stream_position (& mut self) -> std :: io :: Result < u64 > { (* * self) . stream_position () } }
    };
}

impl_71!();