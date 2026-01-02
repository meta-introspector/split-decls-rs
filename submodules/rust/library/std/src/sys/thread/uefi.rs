mkuse!{use crate :: io ;}
mkuse!{use crate :: num :: NonZero ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkuse!{use crate :: time :: Duration ;}

macro_rules! available_parallelism_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function available_parallelism in module {}", module_path!());
    };
}

mkfn!{
    available_parallelism_introspect!();
    pub fn available_parallelism () -> io :: Result < NonZero < usize > > { Ok (NonZero :: new (1) . unwrap ()) }
}

macro_rules! sleep_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sleep in module {}", module_path!());
    };
}

mkfn!{
    sleep_introspect!();
    pub fn sleep (dur : Duration) { let boot_services : NonNull < r_efi :: efi :: BootServices > = crate :: os :: uefi :: env :: boot_services () . expect ("can't sleep") . cast () ; let mut dur_ms = dur . as_micros () ; if dur . subsec_nanos () % 1000 > 0 { dur_ms += 1 ; } while dur_ms > 0 { let ms = crate :: cmp :: min (dur_ms , usize :: MAX as u128) ; let _ = unsafe { ((* boot_services . as_ptr ()) . stall) (ms as usize) } ; dur_ms -= ms ; } }
}