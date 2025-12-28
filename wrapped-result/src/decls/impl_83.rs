macro_rules! deps {
    () => {
        WIN32_ERROR!();
        HRESULT!();
        Result!();
        HeapString!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl HRESULT { # [doc = " Returns [`true`] if `self` is a success code."] # [inline] pub const fn is_ok (self) -> bool { self . 0 >= 0 } # [doc = " Returns [`true`] if `self` is a failure code."] # [inline] pub const fn is_err (self) -> bool { ! self . is_ok () } # [doc = " Asserts that `self` is a success code."] # [doc = ""] # [doc = " This will invoke the [`panic!`] macro if `self` is a failure code and display"] # [doc = " the [`HRESULT`] value for diagnostics."] # [inline] # [track_caller] pub fn unwrap (self) { assert ! (self . is_ok () , "HRESULT 0x{:X}" , self . 0) ; } # [doc = " Converts the [`HRESULT`] to [`Result<()>`][Result<_>]."] # [inline] pub fn ok (self) -> Result < () > { if self . is_ok () { Ok (()) } else { Err (self . into ()) } } # [doc = " Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]"] # [doc = " converted to [`Result<T>`]."] # [inline] pub fn map < F , T > (self , op : F) -> Result < T > where F : FnOnce () -> T , { self . ok () ? ; Ok (op ()) } # [doc = " Calls `op` if `self` is a success code, otherwise returns [`HRESULT`]"] # [doc = " converted to [`Result<T>`]."] # [inline] pub fn and_then < F , T > (self , op : F) -> Result < T > where F : FnOnce () -> Result < T > , { self . ok () ? ; op () } # [doc = " The error message describing the error."] pub fn message (self) -> String { # [cfg (windows)] { let mut message = HeapString :: default () ; let mut code = self . 0 ; let mut module = core :: ptr :: null_mut () ; let mut flags = FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS ; unsafe { if self . 0 & 0x1000_0000 == 0x1000_0000 { code ^= 0x1000_0000 ; flags |= FORMAT_MESSAGE_FROM_HMODULE ; module = LoadLibraryExA (c"ntdll.dll" . as_ptr () as _ , core :: ptr :: null_mut () , LOAD_LIBRARY_SEARCH_DEFAULT_DIRS ,) ; } let size = FormatMessageW (flags , module as _ , code as _ , 0 , & mut message . 0 as * mut _ as * mut _ , 0 , core :: ptr :: null () ,) ; if ! message . 0 . is_null () && size > 0 { String :: from_utf16_lossy (wide_trim_end (core :: slice :: from_raw_parts (message . 0 , size as usize ,))) } else { String :: default () } } } # [cfg (not (windows))] { return alloc :: format ! ("0x{:08x}" , self . 0 as u32) ; } } # [doc = " Creates a new `HRESULT` from the Win32 error code returned by `GetLastError()`."] pub fn from_thread () -> Self { # [cfg (windows)] { WIN32_ERROR :: from_thread () . into () } # [cfg (not (windows))] { unimplemented ! () } } }
    };
}

impl_83!();