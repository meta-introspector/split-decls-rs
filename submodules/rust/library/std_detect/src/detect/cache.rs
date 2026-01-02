mkuse!{use core :: sync :: atomic :: { AtomicUsize , Ordering } ;}

macro_rules! set_bit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_bit in module {}", module_path!());
    };
}

mkfn!{
    set_bit_introspect!();
    # [doc = " Sets the `bit` of `x`."] # [inline] const fn set_bit (x : u128 , bit : u32) -> u128 { x | 1 << bit }
}

macro_rules! test_bit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_bit in module {}", module_path!());
    };
}

mkfn!{
    test_bit_introspect!();
    # [doc = " Tests the `bit` of `x`."] # [inline] const fn test_bit (x : u128 , bit : u32) -> bool { x & (1 << bit) != 0 }
}

macro_rules! unset_bit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unset_bit in module {}", module_path!());
    };
}

mkfn!{
    unset_bit_introspect!();
    # [doc = " Unset the `bit of `x`."] # [inline] const fn unset_bit (x : u128 , bit : u32) -> u128 { x & ! (1 << bit) }
}
mkitem!{# [doc = " Maximum number of features that can be cached."] const CACHE_CAPACITY : u32 = 93 ;}
mkitem!{mkstruct!{# [doc = " This type is used to initialize the cache"] # [derive (Copy , Clone , Default , PartialEq , Eq)] pub (crate) struct Initializer (u128) ;}}
mkitem!{mkimpl!{impl Initializer { # [doc = " Tests the `bit` of the cache."] # [inline] pub (crate) fn test (self , bit : u32) -> bool { debug_assert ! (bit < CACHE_CAPACITY , "too many features, time to increase the cache size!") ; test_bit (self . 0 , bit) } # [doc = " Sets the `bit` of the cache."] # [inline] pub (crate) fn set (& mut self , bit : u32) { debug_assert ! (bit < CACHE_CAPACITY , "too many features, time to increase the cache size!") ; let v = self . 0 ; self . 0 = set_bit (v , bit) ; } # [doc = " Unsets the `bit` of the cache."] # [inline] pub (crate) fn unset (& mut self , bit : u32) { debug_assert ! (bit < CACHE_CAPACITY , "too many features, time to increase the cache size!") ; let v = self . 0 ; self . 0 = unset_bit (v , bit) ; } }}}
mkitem!{# [doc = " This global variable is a cache of the features supported by the CPU."] static CACHE : [Cache ; 3] = [Cache :: uninitialized () , Cache :: uninitialized () , Cache :: uninitialized ()] ;}
mkitem!{mkstruct!{# [doc = " Feature cache with capacity for `size_of::<usize>() * 8 - 1` features."] # [doc = ""] # [doc = " Note: 0 is used to represent an uninitialized cache, and (at least) the most"] # [doc = " significant bit is set on any cache which has been initialized."] # [doc = ""] # [doc = " Note: we use `Relaxed` atomic operations, because we are only interested in"] # [doc = " the effects of operations on a single memory location. That is, we only need"] # [doc = " \"modification order\", and not the full-blown \"happens before\"."] struct Cache (AtomicUsize) ;}}
mkitem!{mkimpl!{impl Cache { const CAPACITY : u32 = (core :: mem :: size_of :: < usize > () * 8 - 1) as u32 ; const MASK : usize = (1 << Cache :: CAPACITY) - 1 ; const INITIALIZED_BIT : usize = 1usize << Cache :: CAPACITY ; # [doc = " Creates an uninitialized cache."] # [allow (clippy :: declare_interior_mutable_const)] const fn uninitialized () -> Self { Cache (AtomicUsize :: new (0)) } # [doc = " Is the `bit` in the cache set? Returns `None` if the cache has not been initialized."] # [inline] pub (crate) fn test (& self , bit : u32) -> Option < bool > { let cached = self . 0 . load (Ordering :: Relaxed) ; if cached == 0 { None } else { Some (test_bit (cached as u128 , bit)) } } # [doc = " Initializes the cache."] # [inline] fn initialize (& self , value : usize) -> usize { debug_assert_eq ! ((value & ! Cache :: MASK) , 0) ; self . 0 . store (value | Cache :: INITIALIZED_BIT , Ordering :: Relaxed) ; value } }}}
mkitem!{cfg_select ! { feature = "std_detect_env_override" => { # [inline] fn disable_features (disable : & [u8] , value : & mut Initializer) { if let Ok (disable) = core :: str :: from_utf8 (disable) { for v in disable . split (" ") { let _ = super :: Feature :: from_str (v) . map (| v | value . unset (v as u32)) ; } } } # [inline] fn initialize (mut value : Initializer) -> Initializer { use core :: ffi :: CStr ; const RUST_STD_DETECT_UNSTABLE : & CStr = c"RUST_STD_DETECT_UNSTABLE" ; cfg_select ! { windows => { use alloc :: vec ; # [link (name = "kernel32")] unsafe extern "system" { fn GetEnvironmentVariableA (name : * const u8 , buffer : * mut u8 , size : u32) -> u32 ; } let len = unsafe { GetEnvironmentVariableA (RUST_STD_DETECT_UNSTABLE . as_ptr () . cast ::< u8 > () , core :: ptr :: null_mut () , 0) } ; if len > 0 { let mut env = vec ! [0 ; len as usize + 1] ; let len = unsafe { GetEnvironmentVariableA (RUST_STD_DETECT_UNSTABLE . as_ptr () . cast ::< u8 > () , env . as_mut_ptr () , len + 1) } ; if len > 0 { disable_features (& env [.. len as usize] , & mut value) ; } } } _ => { let env = unsafe { libc :: getenv (RUST_STD_DETECT_UNSTABLE . as_ptr ()) } ; if ! env . is_null () { let len = unsafe { libc :: strlen (env) } ; let env = unsafe { core :: slice :: from_raw_parts (env as * const u8 , len) } ; disable_features (env , & mut value) ; } } } do_initialize (value) ; value } } _ => { # [inline] fn initialize (value : Initializer) -> Initializer { do_initialize (value) ; value } } }}

macro_rules! do_initialize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function do_initialize in module {}", module_path!());
    };
}

mkfn!{
    do_initialize_introspect!();
    # [inline] fn do_initialize (value : Initializer) { CACHE [0] . initialize ((value . 0) as usize & Cache :: MASK) ; CACHE [1] . initialize ((value . 0 >> Cache :: CAPACITY) as usize & Cache :: MASK) ; CACHE [2] . initialize ((value . 0 >> (2 * Cache :: CAPACITY)) as usize & Cache :: MASK) ; }
}

macro_rules! detect_and_initialize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function detect_and_initialize in module {}", module_path!());
    };
}

mkfn!{
    detect_and_initialize_introspect!();
    # [cold] fn detect_and_initialize () -> Initializer { initialize (super :: os :: detect_features ()) }
}

macro_rules! test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test in module {}", module_path!());
    };
}

mkfn!{
    test_introspect!();
    # [doc = " Tests the `bit` of the storage. If the storage has not been initialized,"] # [doc = " initializes it with the result of `os::detect_features()`."] # [doc = ""] # [doc = " On its first invocation, it detects the CPU features and caches them in the"] # [doc = " `CACHE` global variable as an `AtomicU64`."] # [doc = ""] # [doc = " It uses the `Feature` variant to index into this variable as a bitset. If"] # [doc = " the bit is set, the feature is enabled, and otherwise it is disabled."] # [doc = ""] # [doc = " If the feature `std_detect_env_override` is enabled looks for the env"] # [doc = " variable `RUST_STD_DETECT_UNSTABLE` and uses its content to disable"] # [doc = " Features that would had been otherwise detected."] # [inline] pub (crate) fn test (bit : u32) -> bool { let (relative_bit , idx) = if bit < Cache :: CAPACITY { (bit , 0) } else if bit < 2 * Cache :: CAPACITY { (bit - Cache :: CAPACITY , 1) } else { (bit - 2 * Cache :: CAPACITY , 2) } ; CACHE [idx] . test (relative_bit) . unwrap_or_else (| | detect_and_initialize () . test (bit)) }
}