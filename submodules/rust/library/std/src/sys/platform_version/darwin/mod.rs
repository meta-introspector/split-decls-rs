mkuse!{use self :: core_foundation :: { CFDictionaryRef , CFHandle , CFIndex , CFStringRef , CFTypeRef , kCFAllocatorDefault , kCFPropertyListImmutable , kCFStringEncodingUTF8 , } ;}
mkuse!{use crate :: borrow :: Cow ;}
mkuse!{use crate :: bstr :: ByteStr ;}
mkuse!{use crate :: ffi :: { CStr , c_char } ;}
mkuse!{use crate :: num :: { NonZero , ParseIntError } ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkuse!{use crate :: ptr :: null_mut ;}
mkuse!{use crate :: sync :: atomic :: { AtomicU32 , Ordering } ;}
mkuse!{use crate :: { env , fs } ;}
mkmod!{core_foundation, { 
                getname!(core_foundation);
                getsrc!(core_foundation);
                getpath!(core_foundation);
                get_deps!(core_foundation);
                get_crates!(core_foundation);
                mkinclude!(core_foundation);
                 
            }}
mkmod!{public_extern, { 
                getname!(public_extern);
                getsrc!(public_extern);
                getpath!(public_extern);
                get_deps!(public_extern);
                get_crates!(public_extern);
                mkinclude!(public_extern);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{# [doc = " The version of the operating system."] # [doc = ""] # [doc = " We use a packed u32 here to allow for fast comparisons and to match Mach-O's `LC_BUILD_VERSION`."] type OSVersion = u32 ;}

macro_rules! pack_os_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pack_os_version in module {}", module_path!());
    };
}

mkfn!{
    pack_os_version_introspect!();
    # [doc = " Combine parts of a version into an [`OSVersion`]."] # [doc = ""] # [doc = " The size of the parts are inherently limited by Mach-O's `LC_BUILD_VERSION`."] # [inline] const fn pack_os_version (major : u16 , minor : u8 , patch : u8) -> OSVersion { let (major , minor , patch) = (major as u32 , minor as u32 , patch as u32) ; (major << 16) | (minor << 8) | patch }
}

macro_rules! pack_i32_os_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pack_i32_os_version in module {}", module_path!());
    };
}

mkfn!{
    pack_i32_os_version_introspect!();
    # [doc = " [`pack_os_version`], but takes `i32` and saturates."] # [doc = ""] # [doc = " Instead of using e.g. `major as u16`, which truncates."] # [inline] fn pack_i32_os_version (major : i32 , minor : i32 , patch : i32) -> OSVersion { let major : u16 = major . try_into () . unwrap_or (u16 :: MAX) ; let minor : u8 = minor . try_into () . unwrap_or (u8 :: MAX) ; let patch : u8 = patch . try_into () . unwrap_or (u8 :: MAX) ; pack_os_version (major , minor , patch) }
}

macro_rules! current_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_version in module {}", module_path!());
    };
}

mkfn!{
    current_version_introspect!();
    # [doc = " Get the current OS version, packed according to [`pack_os_version`]."] # [doc = ""] # [doc = " # Semantics"] # [doc = ""] # [doc = " The reported version on macOS might be 10.16 if the SDK version of the binary is less than 11.0."] # [doc = " This is a workaround that Apple implemented to handle applications that assumed that macOS"] # [doc = " versions would always start with \"10\", see:"] # [doc = " <https://github.com/apple-oss-distributions/xnu/blob/xnu-11215.81.4/libsyscall/wrappers/system-version-compat.c>"] # [doc = ""] # [doc = " It _is_ possible to get the real version regardless of the SDK version of the binary, this is"] # [doc = " what Zig does:"] # [doc = " <https://github.com/ziglang/zig/blob/0.13.0/lib/std/zig/system/darwin/macos.zig>"] # [doc = ""] # [doc = " We choose to not do that, and instead follow Apple's behaviour here, and return 10.16 when"] # [doc = " compiled with an older SDK; the user should instead upgrade their tooling."] # [doc = ""] # [doc = " NOTE: `rustc` currently doesn't set the right SDK version when linking with ld64, so this will"] # [doc = " have the wrong behaviour with `-Clinker=ld` on x86_64. But that's a `rustc` bug:"] # [doc = " <https://github.com/rust-lang/rust/issues/129432>"] # [inline] fn current_version () -> OSVersion { static CURRENT_VERSION : AtomicU32 = AtomicU32 :: new (0) ; let version = CURRENT_VERSION . load (Ordering :: Relaxed) ; if version == 0 { let version = lookup_version () . get () ; CURRENT_VERSION . store (version , Ordering :: Relaxed) ; version } else { version } }
}

macro_rules! lookup_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lookup_version in module {}", module_path!());
    };
}

mkfn!{
    lookup_version_introspect!();
    # [doc = " Look up the os version."] # [doc = ""] # [doc = " # Aborts"] # [doc = ""] # [doc = " Aborts if reading or parsing the version fails (or if the system was out of memory)."] # [doc = ""] # [doc = " We deliberately choose to abort, as having this silently return an invalid OS version would be"] # [doc = " impossible for a user to debug."] # [cold] extern "C" fn lookup_version () -> NonZero < OSVersion > { let version = version_from_sysctl () . unwrap_or_else (version_from_plist) ; NonZero :: new (version) . expect ("version cannot be 0.0.0") }
}

macro_rules! version_from_sysctl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function version_from_sysctl in module {}", module_path!());
    };
}

mkfn!{
    version_from_sysctl_introspect!();
    # [doc = " Read the version from `kern.osproductversion` or `kern.iossupportversion`."] # [doc = ""] # [doc = " This is faster than `version_from_plist`, since it doesn't need to invoke `dlsym`."] fn version_from_sysctl () -> Option < OSVersion > { if cfg ! (target_abi = "sim") { return None ; } let sysctl_version = | name : & CStr | { let mut buf : [u8 ; 32] = [0 ; 32] ; let mut size = buf . len () ; let ptr = buf . as_mut_ptr () . cast () ; let ret = unsafe { libc :: sysctlbyname (name . as_ptr () , ptr , & mut size , null_mut () , 0) } ; if ret != 0 { return None ; } let buf = & buf [.. (size - 1)] ; if buf . is_empty () { return None ; } Some (parse_os_version (buf) . unwrap_or_else (| err | { panic ! ("failed parsing version from sysctl ({}): {err}" , ByteStr :: new (buf)) })) } ; if cfg ! (target_os = "ios") { if let Some (ios_support_version) = sysctl_version (c"kern.iossupportversion") { return Some (ios_support_version) ; } if cfg ! (target_abi = "macabi") { return None ; } } sysctl_version (c"kern.osproductversion") }
}

macro_rules! version_from_plist_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function version_from_plist in module {}", module_path!());
    };
}

mkfn!{
    version_from_plist_introspect!();
    # [doc = " Look up the current OS version(s) from `/System/Library/CoreServices/SystemVersion.plist`."] # [doc = ""] # [doc = " More specifically, from the `ProductVersion` and `iOSSupportVersion` keys, and from"] # [doc = " `$IPHONE_SIMULATOR_ROOT/System/Library/CoreServices/SystemVersion.plist` on the simulator."] # [doc = ""] # [doc = " This file was introduced in macOS 10.3, which is well below the minimum supported version by"] # [doc = " `rustc`, which is (at the time of writing) macOS 10.12."] # [doc = ""] # [doc = " # Implementation"] # [doc = ""] # [doc = " We do roughly the same thing in here as `compiler-rt`, and dynamically look up CoreFoundation"] # [doc = " utilities for parsing PLists (to avoid having to re-implement that in here, as pulling in a full"] # [doc = " PList parser into `std` seems costly)."] # [doc = ""] # [doc = " If this is found to be undesirable, we _could_ possibly hack it by parsing the PList manually"] # [doc = " (it seems to use the plain-text \"xml1\" encoding/format in all versions), but that seems brittle."] fn version_from_plist () -> OSVersion { let path = root_relative ("/System/Library/CoreServices/SystemVersion.plist") ; let plist_buffer = fs :: read (& path) . unwrap_or_else (| e | panic ! ("failed reading {path:?}: {e}")) ; let cf_handle = CFHandle :: new () ; parse_version_from_plist (& cf_handle , & plist_buffer) }
}

macro_rules! parse_version_from_plist_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_version_from_plist in module {}", module_path!());
    };
}

mkfn!{
    parse_version_from_plist_introspect!();
    # [doc = " Parse OS version from the given PList."] # [doc = ""] # [doc = " Split out from [`version_from_plist`] to allow for testing."] fn parse_version_from_plist (cf_handle : & CFHandle , plist_buffer : & [u8]) -> OSVersion { let plist_data = unsafe { cf_handle . CFDataCreateWithBytesNoCopy (kCFAllocatorDefault , plist_buffer . as_ptr () , plist_buffer . len () as CFIndex , cf_handle . kCFAllocatorNull () ,) } ; assert ! (! plist_data . is_null () , "failed creating CFData") ; let _plist_data_release = Deferred (| | unsafe { cf_handle . CFRelease (plist_data) }) ; let plist = unsafe { cf_handle . CFPropertyListCreateWithData (kCFAllocatorDefault , plist_data , kCFPropertyListImmutable , null_mut () , null_mut () ,) } ; assert ! (! plist . is_null () , "failed reading PList in SystemVersion.plist") ; let _plist_release = Deferred (| | unsafe { cf_handle . CFRelease (plist) }) ; assert_eq ! (unsafe { cf_handle . CFGetTypeID (plist) } , unsafe { cf_handle . CFDictionaryGetTypeID () } , "SystemVersion.plist did not contain a dictionary at the top level") ; let plist : CFDictionaryRef = plist . cast () ; if cfg ! (target_os = "ios") { if let Some (ios_support_version) = unsafe { string_version_key (cf_handle , plist , c"iOSSupportVersion") } { return ios_support_version ; } if cfg ! (target_abi = "macabi") { panic ! ("expected iOSSupportVersion in SystemVersion.plist") ; } } unsafe { string_version_key (cf_handle , plist , c"ProductVersion") } . expect ("expected ProductVersion in SystemVersion.plist") }
}

macro_rules! string_version_key_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function string_version_key in module {}", module_path!());
    };
}

mkfn!{
    string_version_key_introspect!();
    # [doc = " Look up a string key in a CFDictionary, and convert it to an [`OSVersion`]."] unsafe fn string_version_key (cf_handle : & CFHandle , plist : CFDictionaryRef , lookup_key : & CStr ,) -> Option < OSVersion > { let cf_lookup_key = unsafe { cf_handle . CFStringCreateWithCStringNoCopy (kCFAllocatorDefault , lookup_key . as_ptr () , kCFStringEncodingUTF8 , cf_handle . kCFAllocatorNull () ,) } ; assert ! (! cf_lookup_key . is_null () , "failed creating CFString") ; let _lookup_key_release = Deferred (| | unsafe { cf_handle . CFRelease (cf_lookup_key) }) ; let value : CFTypeRef = unsafe { cf_handle . CFDictionaryGetValue (plist , cf_lookup_key) } . cast_mut () ; if value . is_null () { return None ; } assert_eq ! (unsafe { cf_handle . CFGetTypeID (value) } , unsafe { cf_handle . CFStringGetTypeID () } , "key in SystemVersion.plist must be a string") ; let value : CFStringRef = value . cast () ; let mut version_str = [0u8 ; 32] ; let ret = unsafe { cf_handle . CFStringGetCString (value , version_str . as_mut_ptr () . cast :: < c_char > () , version_str . len () as CFIndex , kCFStringEncodingUTF8 ,) } ; assert_ne ! (ret , 0 , "failed getting string from CFString") ; let version_str = CStr :: from_bytes_until_nul (& version_str) . expect ("failed converting CFString to CStr") ; Some (parse_os_version (version_str . to_bytes ()) . unwrap_or_else (| err | { panic ! ("failed parsing version from PList ({}): {err}" , ByteStr :: new (version_str . to_bytes ())) })) }
}

macro_rules! parse_os_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_os_version in module {}", module_path!());
    };
}

mkfn!{
    parse_os_version_introspect!();
    # [doc = " Parse an OS version from a bytestring like b\"10.1\" or b\"14.3.7\"."] fn parse_os_version (version : & [u8]) -> Result < OSVersion , ParseIntError > { if let Some ((major , minor)) = version . split_once (| & b | b == b'.') { let major = u16 :: from_ascii (major) ? ; if let Some ((minor , patch)) = minor . split_once (| & b | b == b'.') { let minor = u8 :: from_ascii (minor) ? ; let patch = u8 :: from_ascii (patch) ? ; Ok (pack_os_version (major , minor , patch)) } else { let minor = u8 :: from_ascii (minor) ? ; Ok (pack_os_version (major , minor , 0)) } } else { let major = u16 :: from_ascii (version) ? ; Ok (pack_os_version (major , 0 , 0)) } }
}

macro_rules! root_relative_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function root_relative in module {}", module_path!());
    };
}

mkfn!{
    root_relative_introspect!();
    # [doc = " Get a path relative to the root directory in which all files for the current env are located."] fn root_relative (path : & str) -> Cow < '_ , Path > { if cfg ! (target_abi = "sim") { let mut root = PathBuf :: from (env :: var_os ("IPHONE_SIMULATOR_ROOT") . expect ("environment variable `IPHONE_SIMULATOR_ROOT` must be set when executing under simulator" ,)) ; root . push (Path :: new (path) . strip_prefix ("/") . unwrap ()) ; root . into () } else { Path :: new (path) . into () } }
}
mkitem!{mkstruct!{struct Deferred < F : FnMut () > (F) ;}}
mkitem!{mkimpl!{impl < F : FnMut () > Drop for Deferred < F > { fn drop (& mut self) { (self . 0) () ; } }}}