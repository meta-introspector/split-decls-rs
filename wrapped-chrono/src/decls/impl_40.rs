macro_rules! deps {
    () => {
        Weekday!();
        IsoWeek!();
        Datelike!();
        TimeZone!();
        Date!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < Tz : TimeZone > Datelike for Date < Tz > { # [inline] fn year (& self) -> i32 { self . naive_local () . year () } # [inline] fn month (& self) -> u32 { self . naive_local () . month () } # [inline] fn month0 (& self) -> u32 { self . naive_local () . month0 () } # [inline] fn day (& self) -> u32 { self . naive_local () . day () } # [inline] fn day0 (& self) -> u32 { self . naive_local () . day0 () } # [inline] fn ordinal (& self) -> u32 { self . naive_local () . ordinal () } # [inline] fn ordinal0 (& self) -> u32 { self . naive_local () . ordinal0 () } # [inline] fn weekday (& self) -> Weekday { self . naive_local () . weekday () } # [inline] fn iso_week (& self) -> IsoWeek { self . naive_local () . iso_week () } # [inline] fn with_year (& self , year : i32) -> Option < Date < Tz > > { map_local (self , | date | date . with_year (year)) } # [inline] fn with_month (& self , month : u32) -> Option < Date < Tz > > { map_local (self , | date | date . with_month (month)) } # [inline] fn with_month0 (& self , month0 : u32) -> Option < Date < Tz > > { map_local (self , | date | date . with_month0 (month0)) } # [inline] fn with_day (& self , day : u32) -> Option < Date < Tz > > { map_local (self , | date | date . with_day (day)) } # [inline] fn with_day0 (& self , day0 : u32) -> Option < Date < Tz > > { map_local (self , | date | date . with_day0 (day0)) } # [inline] fn with_ordinal (& self , ordinal : u32) -> Option < Date < Tz > > { map_local (self , | date | date . with_ordinal (ordinal)) } # [inline] fn with_ordinal0 (& self , ordinal0 : u32) -> Option < Date < Tz > > { map_local (self , | date | date . with_ordinal0 (ordinal0)) } }
    };
}

impl_40!()