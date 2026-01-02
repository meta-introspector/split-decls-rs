mkitem!{# [cfg (any (target_arch = "x86" , target_arch = "arm" , target_arch = "loongarch32" , target_arch = "m68k" , target_arch = "mips" , target_arch = "mips32r6" , target_arch = "powerpc" , target_arch = "csky" , target_arch = "powerpc64"))] const MIN_ALIGN : usize = 8 ;}
mkitem!{# [cfg (any (target_arch = "x86_64" , target_arch = "aarch64" , target_arch = "loongarch64" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "s390x" , target_arch = "sparc64"))] const MIN_ALIGN : usize = 16 ;}
mkitem!{mkstruct!{pub struct System ;}}
mkmod!{realloc_fallback, { 
                getname!(realloc_fallback);
                getsrc!(realloc_fallback);
                getpath!(realloc_fallback);
                get_deps!(realloc_fallback);
                get_crates!(realloc_fallback);
                mkinclude!(realloc_fallback);
                mkuse!{use core :: alloc :: { GlobalAlloc , Layout } ;}
mkuse!{use core :: cmp ;}
mkuse!{use core :: ptr ;}
mkitem!{mkimpl!{impl super :: System { pub (crate) unsafe fn realloc_fallback (& self , ptr : * mut u8 , old_layout : Layout , new_size : usize) -> * mut u8 { let new_layout = Layout :: from_size_align_unchecked (new_size , old_layout . align ()) ; let new_ptr = GlobalAlloc :: alloc (self , new_layout) ; if ! new_ptr . is_null () { let size = cmp :: min (old_layout . size () , new_size) ; ptr :: copy_nonoverlapping (ptr , new_ptr , size) ; GlobalAlloc :: dealloc (self , ptr , old_layout) ; } new_ptr } }}} 
            }}
mkmod!{platform, { 
                getname!(platform);
                getsrc!(platform);
                getpath!(platform);
                get_deps!(platform);
                get_crates!(platform);
                mkinclude!(platform);
                mkmod!{libc, { 
                getname!(libc);
                getsrc!(libc);
                getpath!(libc);
                get_deps!(libc);
                get_crates!(libc);
                mkinclude!(libc);
                mkuse!{use core :: ffi :: { c_void , c_int } ;}
mkitem!{# [link (name = "c")] extern "C" { pub fn malloc (size : usize) -> * mut c_void ; pub fn realloc (ptr : * mut c_void , size : usize) -> * mut c_void ; pub fn calloc (nmemb : usize , size : usize) -> * mut c_void ; pub fn free (ptr : * mut u8) ; pub fn posix_memalign (memptr : * mut * mut c_void , alignment : usize , size : usize) -> c_int ; }} 
            }}
mkuse!{use core :: ptr ;}
mkuse!{use MIN_ALIGN ;}
mkuse!{use System ;}
mkuse!{use core :: alloc :: { GlobalAlloc , Layout } ;}
mkitem!{mkimpl!{unsafe impl GlobalAlloc for System { # [inline] unsafe fn alloc (& self , layout : Layout) -> * mut u8 { if layout . align () <= MIN_ALIGN && layout . align () <= layout . size () { libc :: malloc (layout . size ()) as * mut u8 } else { # [cfg (target_os = "macos")] { if layout . align () > (1 << 31) { return ptr :: null_mut () } } aligned_malloc (& layout) } } # [inline] unsafe fn alloc_zeroed (& self , layout : Layout) -> * mut u8 { if layout . align () <= MIN_ALIGN && layout . align () <= layout . size () { libc :: calloc (layout . size () , 1) as * mut u8 } else { let ptr = self . alloc (layout . clone ()) ; if ! ptr . is_null () { ptr :: write_bytes (ptr , 0 , layout . size ()) ; } ptr } } # [inline] unsafe fn dealloc (& self , ptr : * mut u8 , _layout : Layout) { libc :: free (ptr as * mut _) } # [inline] unsafe fn realloc (& self , ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { if layout . align () <= MIN_ALIGN && layout . align () <= new_size { libc :: realloc (ptr as * mut _ , new_size) as * mut u8 } else { self . realloc_fallback (ptr , layout , new_size) } } }}}

macro_rules! aligned_malloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aligned_malloc in module {}", module_path!());
    };
}

mkfn!{
    aligned_malloc_introspect!();
    # [cfg (any (target_os = "android" , target_os = "hermit" , target_os = "redox" , target_os = "solaris"))] # [inline] unsafe fn aligned_malloc (layout : & Layout) -> * mut u8 { libc :: memalign (layout . align () , layout . size ()) as * mut u8 }
}

macro_rules! aligned_malloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function aligned_malloc in module {}", module_path!());
    };
}

mkfn!{
    aligned_malloc_introspect!();
    # [cfg (not (any (target_os = "android" , target_os = "hermit" , target_os = "redox" , target_os = "solaris")))] # [inline] unsafe fn aligned_malloc (layout : & Layout) -> * mut u8 { let mut out = ptr :: null_mut () ; let ret = libc :: posix_memalign (& mut out , layout . align () , layout . size ()) ; if ret != 0 { ptr :: null_mut () } else { out as * mut u8 } }
} 
            }}
mkmod!{platform, { 
                getname!(platform);
                getsrc!(platform);
                getpath!(platform);
                get_deps!(platform);
                get_crates!(platform);
                mkinclude!(platform);
                mkuse!{use MIN_ALIGN ;}
mkuse!{use System ;}
mkuse!{use core :: alloc :: { GlobalAlloc , Layout } ;}
mkitem!{type LPVOID = * mut u8 ;}
mkitem!{type HANDLE = LPVOID ;}
mkitem!{type SIZE_T = usize ;}
mkitem!{type DWORD = u32 ;}
mkitem!{type BOOL = i32 ;}
mkitem!{extern "system" { fn GetProcessHeap () -> HANDLE ; fn HeapAlloc (hHeap : HANDLE , dwFlags : DWORD , dwBytes : SIZE_T) -> LPVOID ; fn HeapReAlloc (hHeap : HANDLE , dwFlags : DWORD , lpMem : LPVOID , dwBytes : SIZE_T) -> LPVOID ; fn HeapFree (hHeap : HANDLE , dwFlags : DWORD , lpMem : LPVOID) -> BOOL ; fn GetLastError () -> DWORD ; }}
mkitem!{mkstruct!{# [repr (C)] struct Header (* mut u8) ;}}
mkitem!{const HEAP_ZERO_MEMORY : DWORD = 0x00000008 ;}

macro_rules! get_header_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_header in module {}", module_path!());
    };
}

mkfn!{
    get_header_introspect!();
    unsafe fn get_header < 'a > (ptr : * mut u8) -> & 'a mut Header { & mut * (ptr as * mut Header) . sub (1) }
}

macro_rules! align_ptr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function align_ptr in module {}", module_path!());
    };
}

mkfn!{
    align_ptr_introspect!();
    unsafe fn align_ptr (ptr : * mut u8 , align : usize) -> * mut u8 { let aligned = ptr . add (align - (ptr as usize & (align - 1))) ; * get_header (aligned) = Header (ptr) ; aligned }
}

macro_rules! allocate_with_flags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function allocate_with_flags in module {}", module_path!());
    };
}

mkfn!{
    allocate_with_flags_introspect!();
    # [inline] unsafe fn allocate_with_flags (layout : Layout , flags : DWORD) -> * mut u8 { let ptr = if layout . align () <= MIN_ALIGN { HeapAlloc (GetProcessHeap () , flags , layout . size ()) } else { let size = layout . size () + layout . align () ; let ptr = HeapAlloc (GetProcessHeap () , flags , size) ; if ptr . is_null () { ptr } else { align_ptr (ptr , layout . align ()) } } ; ptr as * mut u8 }
}
mkitem!{mkimpl!{unsafe impl GlobalAlloc for System { # [inline] unsafe fn alloc (& self , layout : Layout) -> * mut u8 { allocate_with_flags (layout , 0) } # [inline] unsafe fn alloc_zeroed (& self , layout : Layout) -> * mut u8 { allocate_with_flags (layout , HEAP_ZERO_MEMORY) } # [inline] unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { if layout . align () <= MIN_ALIGN { let err = HeapFree (GetProcessHeap () , 0 , ptr as LPVOID) ; debug_assert ! (err != 0 , "Failed to free heap memory: {}" , GetLastError ()) ; } else { let header = get_header (ptr) ; let err = HeapFree (GetProcessHeap () , 0 , header . 0 as LPVOID) ; debug_assert ! (err != 0 , "Failed to free heap memory: {}" , GetLastError ()) ; } } # [inline] unsafe fn realloc (& self , ptr : * mut u8 , layout : Layout , new_size : usize) -> * mut u8 { if layout . align () <= MIN_ALIGN { HeapReAlloc (GetProcessHeap () , 0 , ptr as LPVOID , new_size) as * mut u8 } else { self . realloc_fallback (ptr , layout , new_size) } } }}} 
            }}