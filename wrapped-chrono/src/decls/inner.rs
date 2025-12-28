macro_rules! deps {
    () => {
        MappedLocalTime!();
        Timelike!();
        Date!();
        FixedOffset!();
        NaiveDateTime!();
        Datelike!();
    };
}

macro_rules! inner {
    () => {
        deps!();
        # [cfg (all (target_arch = "wasm32" , feature = "wasmbind" , not (any (target_os = "emscripten" , target_os = "wasi" , target_os = "linux"))))] mod inner { use crate :: { Datelike , FixedOffset , MappedLocalTime , NaiveDateTime , Timelike } ; pub (super) fn offset_from_utc_datetime (utc : & NaiveDateTime) -> MappedLocalTime < FixedOffset > { let offset = js_sys :: Date :: from (utc . and_utc ()) . get_timezone_offset () ; MappedLocalTime :: Single (FixedOffset :: west_opt ((offset as i32) * 60) . unwrap ()) } pub (super) fn offset_from_local_datetime (local : & NaiveDateTime ,) -> MappedLocalTime < FixedOffset > { let mut year = local . year () ; if year < 100 { let shift_cycles = (year - 100) . div_euclid (400) ; year -= shift_cycles * 400 ; } let js_date = js_sys :: Date :: new_with_year_month_day_hr_min_sec (year as u32 , local . month0 () as i32 , local . day () as i32 , local . hour () as i32 , local . minute () as i32 , local . second () as i32 ,) ; let offset = js_date . get_timezone_offset () ; MappedLocalTime :: Single (FixedOffset :: west_opt ((offset as i32) * 60) . unwrap ()) } }
    };
}

inner!()