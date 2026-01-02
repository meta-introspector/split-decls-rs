mkuse!{use core :: arch :: asm ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkitem!{mkstruct!{# [doc = " Wrapper struct to force 16-byte alignment."] # [repr (align (16))] # [unstable (feature = "sgx_platform" , issue = "56975")] pub struct Align16 < T > (pub T) ;}}
mkitem!{mkstruct!{# [doc = " Wrapper struct to force 128-byte alignment."] # [repr (align (128))] # [unstable (feature = "sgx_platform" , issue = "56975")] pub struct Align128 < T > (pub T) ;}}
mkitem!{mkstruct!{# [doc = " Wrapper struct to force 512-byte alignment."] # [repr (align (512))] # [unstable (feature = "sgx_platform" , issue = "56975")] pub struct Align512 < T > (pub T) ;}}
mkitem!{const ENCLU_EREPORT : u32 = 0 ;}
mkitem!{const ENCLU_EGETKEY : u32 = 1 ;}

macro_rules! egetkey_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function egetkey in module {}", module_path!());
    };
}

mkfn!{
    egetkey_introspect!();
    # [doc = " Call the `EGETKEY` instruction to obtain a 128-bit secret key."] # [unstable (feature = "sgx_platform" , issue = "56975")] pub fn egetkey (request : & Align512 < [u8 ; 512] >) -> Result < Align16 < [u8 ; 16] > , u32 > { unsafe { let mut out = MaybeUninit :: uninit () ; let error ; asm ! ("xchg %rbx, {0}" , "enclu" , "mov {0}, %rbx" , inout (reg) request => _ , inlateout ("eax") ENCLU_EGETKEY => error , in ("rcx") out . as_mut_ptr () , options (att_syntax , nostack) ,) ; match error { 0 => Ok (out . assume_init ()) , err => Err (err) , } } }
}

macro_rules! ereport_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ereport in module {}", module_path!());
    };
}

mkfn!{
    ereport_introspect!();
    # [doc = " Call the `EREPORT` instruction."] # [doc = ""] # [doc = " This creates a cryptographic report describing the contents of the current"] # [doc = " enclave. The report may be verified by the enclave described in"] # [doc = " `targetinfo`."] # [unstable (feature = "sgx_platform" , issue = "56975")] pub fn ereport (targetinfo : & Align512 < [u8 ; 512] > , reportdata : & Align128 < [u8 ; 64] > ,) -> Align512 < [u8 ; 432] > { unsafe { let mut report = MaybeUninit :: uninit () ; asm ! ("xchg %rbx, {0}" , "enclu" , "mov {0}, %rbx" , inout (reg) targetinfo => _ , in ("eax") ENCLU_EREPORT , in ("rcx") reportdata , in ("rdx") report . as_mut_ptr () , options (att_syntax , preserves_flags , nostack) ,) ; report . assume_init () } }
}