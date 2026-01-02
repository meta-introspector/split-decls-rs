mkuse!{use core :: arch :: global_asm ;}
mkuse!{use core :: sync :: atomic :: { Atomic , AtomicUsize , Ordering } ;}
mkuse!{use crate :: io :: Write ;}
mkmod!{panic, { 
                getname!(panic);
                getsrc!(panic);
                getpath!(panic);
                get_deps!(panic);
                get_crates!(panic);
                mkinclude!(panic);
                 
            }}
mkmod!{reloc, { 
                getname!(reloc);
                getsrc!(reloc);
                getpath!(reloc);
                get_deps!(reloc);
                get_crates!(reloc);
                mkinclude!(reloc);
                 
            }}
mkmod!{mem, { 
                getname!(mem);
                getsrc!(mem);
                getpath!(mem);
                get_deps!(mem);
                get_crates!(mem);
                mkinclude!(mem);
                 
            }}
mkmod!{thread, { 
                getname!(thread);
                getsrc!(thread);
                getpath!(thread);
                get_deps!(thread);
                get_crates!(thread);
                mkinclude!(thread);
                 
            }}
mkmod!{tls, { 
                getname!(tls);
                getsrc!(tls);
                getpath!(tls);
                get_deps!(tls);
                get_crates!(tls);
                mkinclude!(tls);
                 
            }}
mkmod!{usercalls, { 
                getname!(usercalls);
                getsrc!(usercalls);
                getpath!(usercalls);
                get_deps!(usercalls);
                get_crates!(usercalls);
                mkinclude!(usercalls);
                 
            }}
mkitem!{# [cfg (not (test))] global_asm ! (include_str ! ("entry.S") , options (att_syntax)) ;}
mkitem!{mkstruct!{# [repr (C)] struct EntryReturn (u64 , u64) ;}}

macro_rules! tcs_init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tcs_init in module {}", module_path!());
    };
}

mkfn!{
    tcs_init_introspect!();
    # [cfg (not (test))] # [unsafe (no_mangle)] unsafe extern "C" fn tcs_init (secondary : bool) { const UNINIT : usize = 0 ; const BUSY : usize = 1 ; const DONE : usize = 2 ; static RELOC_STATE : Atomic < usize > = AtomicUsize :: new (UNINIT) ; if secondary && RELOC_STATE . load (Ordering :: Relaxed) != DONE { rtabort ! ("Entered secondary TCS before main TCS!") } match RELOC_STATE . compare_exchange (UNINIT , BUSY , Ordering :: Acquire , Ordering :: Acquire) { Ok (_) => { reloc :: relocate_elf_rela () ; RELOC_STATE . store (DONE , Ordering :: Release) ; } Err (BUSY) => { while RELOC_STATE . load (Ordering :: Acquire) == BUSY { core :: hint :: spin_loop () ; } } Err (DONE) => { } _ => unreachable ! () , } }
}

macro_rules! entry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function entry in module {}", module_path!());
    };
}

mkfn!{
    entry_introspect!();
    # [cfg (not (test))] # [unsafe (no_mangle)] extern "C" fn entry (p1 : u64 , p2 : u64 , p3 : u64 , secondary : bool , p4 : u64 , p5 : u64) -> EntryReturn { let tls = Box :: new (tls :: Tls :: new ()) ; let tls_guard = unsafe { tls . activate () } ; if secondary { let join_notifier = crate :: sys :: thread :: Thread :: entry () ; drop (tls_guard) ; drop (join_notifier) ; EntryReturn (0 , 0) } else { unsafe extern "C" { fn main (argc : isize , argv : * const * const u8) -> isize ; } rtassert ! (p3 == 0) ; rtassert ! (p4 == 0) ; rtassert ! (p5 == 0) ; unsafe { let ret = main (p2 as _ , p1 as _) ; exit_with_code (ret) } } }
}

macro_rules! exit_with_code_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exit_with_code in module {}", module_path!());
    };
}

mkfn!{
    exit_with_code_introspect!();
    pub (super) fn exit_with_code (code : isize) -> ! { if code != 0 { if let Some (mut out) = panic :: SgxPanicOutput :: new () { let _ = write ! (out , "Exited with status code {code}") ; } } usercalls :: exit (code != 0) ; }
}

macro_rules! abort_reentry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function abort_reentry in module {}", module_path!());
    };
}

mkfn!{
    abort_reentry_introspect!();
    # [cfg (not (test))] # [unsafe (no_mangle)] extern "C" fn abort_reentry () -> ! { usercalls :: exit (false) }
}