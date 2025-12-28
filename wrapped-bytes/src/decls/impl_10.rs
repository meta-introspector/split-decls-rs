macro_rules! deps {
    () => {
        Buf!();
        TryGetError!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T : AsRef < [u8] > > Buf for std :: io :: Cursor < T > { # [inline] fn remaining (& self) -> usize { saturating_sub_usize_u64 (self . get_ref () . as_ref () . len () , self . position ()) } # [inline] fn chunk (& self) -> & [u8] { let slice = self . get_ref () . as_ref () ; let pos = min_u64_usize (self . position () , slice . len ()) ; & slice [pos ..] } # [inline] fn advance (& mut self , cnt : usize) { let len = self . get_ref () . as_ref () . len () ; let pos = self . position () ; let max_cnt = saturating_sub_usize_u64 (len , pos) ; if cnt > max_cnt { panic_advance (& TryGetError { requested : cnt , available : max_cnt , }) ; } self . set_position (pos + cnt as u64) ; } }
    };
}

impl_10!()