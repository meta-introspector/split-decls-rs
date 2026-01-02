mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)] pub struct Instant (Duration) ;}}
mkitem!{mkstruct!{# [doc = " When a Timezone is specified, the stored Duration is in UTC. If timezone is unspecified, then"] # [doc = " the timezone is assumed to be in UTC."] # [doc = ""] # [doc = " UEFI SystemTime is stored as Duration from 1900-01-01-00:00:00 with timezone -1440 as anchor"] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug , Hash)] pub struct SystemTime (Duration) ;}}
mkitem!{pub const UNIX_EPOCH : SystemTime = SystemTime :: from_uefi (r_efi :: efi :: Time { year : 1970 , month : 1 , day : 1 , hour : 0 , minute : 0 , second : 0 , nanosecond : 0 , timezone : 0 , daylight : 0 , pad1 : 0 , pad2 : 0 , }) ;}
mkitem!{const MAX_UEFI_TIME : SystemTime = SystemTime :: from_uefi (r_efi :: efi :: Time { year : 9999 , month : 12 , day : 31 , hour : 23 , minute : 59 , second : 59 , nanosecond : 999_999_999 , timezone : 1440 , daylight : 0 , pad1 : 0 , pad2 : 0 , }) ;}
mkitem!{mkimpl!{impl Instant { pub fn now () -> Instant { if let Some (x) = instant_internal :: timestamp_protocol () { return x ; } if let Some (x) = instant_internal :: platform_specific () { return x ; } panic ! ("time not implemented on this platform") } pub fn checked_sub_instant (& self , other : & Instant) -> Option < Duration > { self . 0 . checked_sub (other . 0) } pub fn checked_add_duration (& self , other : & Duration) -> Option < Instant > { Some (Instant (self . 0 . checked_add (* other) ?)) } pub fn checked_sub_duration (& self , other : & Duration) -> Option < Instant > { Some (Instant (self . 0 . checked_sub (* other) ?)) } }}}
mkitem!{mkimpl!{impl SystemTime { pub (crate) const fn from_uefi (t : r_efi :: efi :: Time) -> Self { Self (system_time_internal :: from_uefi (& t)) } # [expect (dead_code)] pub (crate) const fn to_uefi (self , timezone : i16 , daylight : u8) -> Option < r_efi :: efi :: Time > { system_time_internal :: to_uefi (& self . 0 , timezone , daylight) } pub fn now () -> SystemTime { system_time_internal :: now () . unwrap_or_else (| | panic ! ("time not implemented on this platform")) } pub fn sub_time (& self , other : & SystemTime) -> Result < Duration , Duration > { self . 0 . checked_sub (other . 0) . ok_or_else (| | other . 0 - self . 0) } pub fn checked_add_duration (& self , other : & Duration) -> Option < SystemTime > { let temp = Self (self . 0 . checked_add (* other) ?) ; if temp <= MAX_UEFI_TIME { Some (temp) } else { None } } pub fn checked_sub_duration (& self , other : & Duration) -> Option < SystemTime > { self . 0 . checked_sub (* other) . map (Self) } }}}
mkmod!{system_time_internal, { 
                getname!(system_time_internal);
                getsrc!(system_time_internal);
                getpath!(system_time_internal);
                get_deps!(system_time_internal);
                get_crates!(system_time_internal);
                mkinclude!(system_time_internal);
                mkuse!{use r_efi :: efi :: { RuntimeServices , Time } ;}
mkuse!{use super :: super :: helpers ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkitem!{const SECS_IN_MINUTE : u64 = 60 ;}
mkitem!{const SECS_IN_HOUR : u64 = SECS_IN_MINUTE * 60 ;}
mkitem!{const SECS_IN_DAY : u64 = SECS_IN_HOUR * 24 ;}
mkitem!{const TIMEZONE_DELTA : u64 = 1440 * SECS_IN_MINUTE ;}

macro_rules! now_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function now in module {}", module_path!());
    };
}

mkfn!{
    now_introspect!();
    pub fn now () -> Option < SystemTime > { let runtime_services : NonNull < RuntimeServices > = helpers :: runtime_services () ? ; let mut t : MaybeUninit < Time > = MaybeUninit :: uninit () ; let r = unsafe { ((* runtime_services . as_ptr ()) . get_time) (t . as_mut_ptr () , crate :: ptr :: null_mut ()) } ; if r . is_error () { return None ; } let t = unsafe { t . assume_init () } ; Some (SystemTime :: from_uefi (t)) }
}

macro_rules! from_uefi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_uefi in module {}", module_path!());
    };
}

mkfn!{
    from_uefi_introspect!();
    # [doc = " This algorithm is a modified form of the one described in the post"] # [doc = " https://blog.reverberate.org/2020/05/12/optimizing-date-algorithms.html"] # [doc = ""] # [doc = " The changes are to use 1900-01-01-00:00:00 with timezone -1440 as anchor instead of UNIX"] # [doc = " epoch used in the original algorithm."] pub (crate) const fn from_uefi (t : & Time) -> Duration { assert ! (t . month <= 12 && t . month != 0) ; assert ! (t . year >= 1900 && t . year <= 9999) ; assert ! (t . day <= 31 && t . day != 0) ; assert ! (t . second < 60) ; assert ! (t . minute < 60) ; assert ! (t . hour < 24) ; assert ! (t . nanosecond < 1_000_000_000) ; assert ! ((t . timezone <= 1440 && t . timezone >= - 1440) || t . timezone == r_efi :: efi :: UNSPECIFIED_TIMEZONE) ; const YEAR_BASE : u32 = 4800 ; let (m_adj , overflow) : (u32 , bool) = (t . month as u32) . overflowing_sub (3) ; let (carry , adjust) : (u32 , u32) = if overflow { (1 , 12) } else { (0 , 0) } ; let y_adj : u32 = (t . year as u32) + YEAR_BASE - carry ; let month_days : u32 = (m_adj . wrapping_add (adjust) * 62719 + 769) / 2048 ; let leap_days : u32 = y_adj / 4 - y_adj / 100 + y_adj / 400 ; let days : u32 = y_adj * 365 + leap_days + month_days + (t . day as u32 - 1) - 2447065 ; let localtime_epoch : u64 = (days as u64) * SECS_IN_DAY + (t . second as u64) + (t . minute as u64) * SECS_IN_MINUTE + (t . hour as u64) * SECS_IN_HOUR ; let adjusted_localtime_epoc : u64 = localtime_epoch + TIMEZONE_DELTA ; let epoch : u64 = if t . timezone == r_efi :: efi :: UNSPECIFIED_TIMEZONE { adjusted_localtime_epoc } else { adjusted_localtime_epoc . checked_add_signed ((t . timezone as i64) * SECS_IN_MINUTE as i64) . unwrap () } ; Duration :: new (epoch , t . nanosecond) }
}

macro_rules! to_uefi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_uefi in module {}", module_path!());
    };
}

mkfn!{
    to_uefi_introspect!();
    # [doc = " This algorithm is a modified version of the one described in the post:"] # [doc = " https://howardhinnant.github.io/date_algorithms.html#clive_from_days"] # [doc = ""] # [doc = " The changes are to use 1900-01-01-00:00:00 with timezone -1440 as anchor instead of UNIX"] # [doc = " epoch used in the original algorithm."] pub (crate) const fn to_uefi (dur : & Duration , timezone : i16 , daylight : u8) -> Option < Time > { assert ! (timezone <= 1440 && timezone >= - 1440) ; let secs = dur . as_secs () . checked_add_signed ((- timezone as i64) * SECS_IN_MINUTE as i64) . unwrap () ; let Some (secs) = secs . checked_sub (TIMEZONE_DELTA) else { return None } ; let days = secs / SECS_IN_DAY ; let remaining_secs = secs % SECS_IN_DAY ; let z = days + 693901 ; let era = z / 146097 ; let doe = z - (era * 146097) ; let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365 ; let mut y = yoe + era * 400 ; let doy = doe - (365 * yoe + yoe / 4 - yoe / 100) ; let mp = (5 * doy + 2) / 153 ; let d = doy - (153 * mp + 2) / 5 + 1 ; let m = if mp < 10 { mp + 3 } else { mp - 9 } ; if m <= 2 { y += 1 ; } let hour = (remaining_secs / SECS_IN_HOUR) as u8 ; let minute = ((remaining_secs % SECS_IN_HOUR) / SECS_IN_MINUTE) as u8 ; let second = (remaining_secs % SECS_IN_MINUTE) as u8 ; if y >= 1900 && y <= 9999 { Some (Time { year : y as u16 , month : m as u8 , day : d as u8 , hour , minute , second , nanosecond : dur . subsec_nanos () , timezone , daylight , pad1 : 0 , pad2 : 0 , }) } else { None } }
} 
            }}
mkmod!{instant_internal, { 
                getname!(instant_internal);
                getsrc!(instant_internal);
                getpath!(instant_internal);
                get_deps!(instant_internal);
                get_crates!(instant_internal);
                mkinclude!(instant_internal);
                mkuse!{use r_efi :: protocols :: timestamp ;}
mkuse!{use super :: super :: helpers ;}
mkuse!{use super :: * ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicPtr , Ordering } ;}
mkuse!{use crate :: sys_common :: mul_div_u64 ;}
mkitem!{const NS_PER_SEC : u64 = 1_000_000_000 ;}

macro_rules! timestamp_protocol_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function timestamp_protocol in module {}", module_path!());
    };
}

mkfn!{
    timestamp_protocol_introspect!();
    pub fn timestamp_protocol () -> Option < Instant > { fn try_handle (handle : NonNull < crate :: ffi :: c_void >) -> Option < u64 > { let protocol : NonNull < timestamp :: Protocol > = helpers :: open_protocol (handle , timestamp :: PROTOCOL_GUID) . ok () ? ; let mut properties : MaybeUninit < timestamp :: Properties > = MaybeUninit :: uninit () ; let r = unsafe { ((* protocol . as_ptr ()) . get_properties) (properties . as_mut_ptr ()) } ; if r . is_error () { return None ; } let freq = unsafe { properties . assume_init () . frequency } ; let ts = unsafe { ((* protocol . as_ptr ()) . get_timestamp) () } ; Some (mul_div_u64 (ts , NS_PER_SEC , freq)) } static LAST_VALID_HANDLE : Atomic < * mut crate :: ffi :: c_void > = AtomicPtr :: new (crate :: ptr :: null_mut ()) ; if let Some (handle) = NonNull :: new (LAST_VALID_HANDLE . load (Ordering :: Acquire)) { if let Some (ns) = try_handle (handle) { return Some (Instant (Duration :: from_nanos (ns))) ; } } if let Ok (handles) = helpers :: locate_handles (timestamp :: PROTOCOL_GUID) { for handle in handles { if let Some (ns) = try_handle (handle) { LAST_VALID_HANDLE . store (handle . as_ptr () , Ordering :: Release) ; return Some (Instant (Duration :: from_nanos (ns))) ; } } } None }
}

macro_rules! platform_specific_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function platform_specific in module {}", module_path!());
    };
}

mkfn!{
    platform_specific_introspect!();
    pub fn platform_specific () -> Option < Instant > { cfg_select ! { any (target_arch = "x86_64" , target_arch = "x86") => timestamp_rdtsc () . map (Instant) , _ => None , } }
}

macro_rules! timestamp_rdtsc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function timestamp_rdtsc in module {}", module_path!());
    };
}

mkfn!{
    timestamp_rdtsc_introspect!();
    # [cfg (target_arch = "x86_64")] fn timestamp_rdtsc () -> Option < Duration > { static FREQUENCY : crate :: sync :: OnceLock < u64 > = crate :: sync :: OnceLock :: new () ; let freq = FREQUENCY . get_or_try_init (| | { let cpuid = unsafe { crate :: arch :: x86_64 :: __cpuid (0x15) } ; if cpuid . eax == 0 || cpuid . ebx == 0 || cpuid . ecx == 0 { return Err (()) ; } Ok (mul_div_u64 (cpuid . ecx as u64 , cpuid . ebx as u64 , cpuid . eax as u64)) }) . ok () ? ; let ts = unsafe { crate :: arch :: x86_64 :: _rdtsc () } ; let ns = mul_div_u64 (ts , 1000 , * freq) ; Some (Duration :: from_nanos (ns)) }
}

macro_rules! timestamp_rdtsc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function timestamp_rdtsc in module {}", module_path!());
    };
}

mkfn!{
    timestamp_rdtsc_introspect!();
    # [cfg (target_arch = "x86")] fn timestamp_rdtsc () -> Option < Duration > { static FREQUENCY : crate :: sync :: OnceLock < u64 > = crate :: sync :: OnceLock :: new () ; let freq = FREQUENCY . get_or_try_init (| | { let cpuid = unsafe { crate :: arch :: x86 :: __cpuid (0x15) } ; if cpuid . eax == 0 || cpuid . ebx == 0 || cpuid . ecx == 0 { return Err (()) ; } Ok (mul_div_u64 (cpuid . ecx as u64 , cpuid . ebx as u64 , cpuid . eax as u64)) }) . ok () ? ; let ts = unsafe { crate :: arch :: x86 :: _rdtsc () } ; let ns = mul_div_u64 (ts , 1000 , * freq) ; Some (Duration :: from_nanos (ns)) }
} 
            }}