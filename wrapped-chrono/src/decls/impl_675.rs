macro_rules! deps {
    () => {
        MappedLocalTime!();
        NaiveTime!();
        Date!();
        TimeZone!();
        DateTime!();
    };
}

macro_rules! impl_675 {
    () => {
        deps!();
        # [allow (deprecated)] impl < Tz : TimeZone > MappedLocalTime < Date < Tz > > { # [doc = " Makes a new `DateTime` from the current date and given `NaiveTime`."] # [doc = " The offset in the current date is preserved."] # [doc = ""] # [doc = " Propagates any error. Ambiguous result would be discarded."] # [inline] # [must_use] pub fn and_time (self , time : NaiveTime) -> MappedLocalTime < DateTime < Tz > > { match self { MappedLocalTime :: Single (d) => { d . and_time (time) . map_or (MappedLocalTime :: None , MappedLocalTime :: Single) } _ => MappedLocalTime :: None , } } # [doc = " Makes a new `DateTime` from the current date, hour, minute and second."] # [doc = " The offset in the current date is preserved."] # [doc = ""] # [doc = " Propagates any error. Ambiguous result would be discarded."] # [inline] # [must_use] pub fn and_hms_opt (self , hour : u32 , min : u32 , sec : u32) -> MappedLocalTime < DateTime < Tz > > { match self { MappedLocalTime :: Single (d) => { d . and_hms_opt (hour , min , sec) . map_or (MappedLocalTime :: None , MappedLocalTime :: Single) } _ => MappedLocalTime :: None , } } # [doc = " Makes a new `DateTime` from the current date, hour, minute, second and millisecond."] # [doc = " The millisecond part can exceed 1,000 in order to represent the leap second."] # [doc = " The offset in the current date is preserved."] # [doc = ""] # [doc = " Propagates any error. Ambiguous result would be discarded."] # [inline] # [must_use] pub fn and_hms_milli_opt (self , hour : u32 , min : u32 , sec : u32 , milli : u32 ,) -> MappedLocalTime < DateTime < Tz > > { match self { MappedLocalTime :: Single (d) => d . and_hms_milli_opt (hour , min , sec , milli) . map_or (MappedLocalTime :: None , MappedLocalTime :: Single) , _ => MappedLocalTime :: None , } } # [doc = " Makes a new `DateTime` from the current date, hour, minute, second and microsecond."] # [doc = " The microsecond part can exceed 1,000,000 in order to represent the leap second."] # [doc = " The offset in the current date is preserved."] # [doc = ""] # [doc = " Propagates any error. Ambiguous result would be discarded."] # [inline] # [must_use] pub fn and_hms_micro_opt (self , hour : u32 , min : u32 , sec : u32 , micro : u32 ,) -> MappedLocalTime < DateTime < Tz > > { match self { MappedLocalTime :: Single (d) => d . and_hms_micro_opt (hour , min , sec , micro) . map_or (MappedLocalTime :: None , MappedLocalTime :: Single) , _ => MappedLocalTime :: None , } } # [doc = " Makes a new `DateTime` from the current date, hour, minute, second and nanosecond."] # [doc = " The nanosecond part can exceed 1,000,000,000 in order to represent the leap second."] # [doc = " The offset in the current date is preserved."] # [doc = ""] # [doc = " Propagates any error. Ambiguous result would be discarded."] # [inline] # [must_use] pub fn and_hms_nano_opt (self , hour : u32 , min : u32 , sec : u32 , nano : u32 ,) -> MappedLocalTime < DateTime < Tz > > { match self { MappedLocalTime :: Single (d) => d . and_hms_nano_opt (hour , min , sec , nano) . map_or (MappedLocalTime :: None , MappedLocalTime :: Single) , _ => MappedLocalTime :: None , } } }
    };
}

impl_675!()