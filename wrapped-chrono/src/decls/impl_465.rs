macro_rules! deps {
    () => {
        YearFlags!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl YearFlags { # [allow (unreachable_pub)] # [doc (hidden)] # [inline] # [must_use] pub const fn from_year (year : i32) -> YearFlags { let year = year . rem_euclid (400) ; YearFlags :: from_year_mod_400 (year) } # [inline] pub (super) const fn from_year_mod_400 (year : i32) -> YearFlags { YEAR_TO_FLAGS [year as usize] } # [inline] pub (super) const fn ndays (& self) -> u32 { let YearFlags (flags) = * self ; 366 - (flags >> 3) as u32 } # [inline] pub (super) const fn isoweek_delta (& self) -> u32 { let YearFlags (flags) = * self ; let mut delta = (flags & 0b0111) as u32 ; if delta < 3 { delta += 7 ; } delta } # [inline] pub (super) const fn nisoweeks (& self) -> u32 { let YearFlags (flags) = * self ; 52 + ((0b0000_0100_0000_0110 >> flags as usize) & 1) } }
    };
}

impl_465!();