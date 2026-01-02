mkuse!{use core :: ffi :: c_void ;}
mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use super :: c ;}
mkitem!{# [doc = " Creates a null-terminated UTF-16 string from a str."] pub macro wide_str ($ str : literal) { { const _ : () = { if core :: slice :: memchr :: memchr (0 , $ str . as_bytes ()) . is_some () { panic ! ("null terminated strings cannot contain interior nulls") ; } } ; crate :: sys :: pal :: windows :: api :: utf16 ! (concat ! ($ str , '\0')) } }}
mkitem!{# [doc = " Creates a UTF-16 string from a str without null termination."] pub macro utf16 ($ str : expr) { { const UTF8 : & str = $ str ; const UTF16_LEN : usize = crate :: sys :: pal :: windows :: api :: utf16_len (UTF8) ; const UTF16 : [u16 ; UTF16_LEN] = crate :: sys :: pal :: windows :: api :: to_utf16 (UTF8) ; & UTF16 } }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}

macro_rules! utf16_len_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function utf16_len in module {}", module_path!());
    };
}

mkfn!{
    utf16_len_introspect!();
    # [doc = " Gets the UTF-16 length of a UTF-8 string, for use in the wide_str macro."] pub const fn utf16_len (s : & str) -> usize { let s = s . as_bytes () ; let mut i = 0 ; let mut len = 0 ; while i < s . len () { let utf8_len = match s [i] . leading_ones () { 0 => 1 , n => n as usize , } ; i += utf8_len ; len += if utf8_len < 4 { 1 } else { 2 } ; } len }
}

macro_rules! to_utf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_utf16 in module {}", module_path!());
    };
}

mkfn!{
    to_utf16_introspect!();
    # [doc = " Const convert UTF-8 to UTF-16, for use in the wide_str macro."] # [doc = ""] # [doc = " Note that this is designed for use in const contexts so is not optimized."] pub const fn to_utf16 < const UTF16_LEN : usize > (s : & str) -> [u16 ; UTF16_LEN] { let mut output = [0_u16 ; UTF16_LEN] ; let mut pos = 0 ; let s = s . as_bytes () ; let mut i = 0 ; while i < s . len () { match s [i] . leading_ones () { 0 => { output [pos] = s [i] as u16 ; i += 1 ; pos += 1 ; } 2 => { output [pos] = ((s [i] as u16 & 0b11111) << 6) | (s [i + 1] as u16 & 0b111111) ; i += 2 ; pos += 1 ; } 3 => { output [pos] = ((s [i] as u16 & 0b1111) << 12) | ((s [i + 1] as u16 & 0b111111) << 6) | (s [i + 2] as u16 & 0b111111) ; i += 3 ; pos += 1 ; } 4 => { let mut c = ((s [i] as u32 & 0b111) << 18) | ((s [i + 1] as u32 & 0b111111) << 12) | ((s [i + 2] as u32 & 0b111111) << 6) | (s [i + 3] as u32 & 0b111111) ; c -= 0x10000 ; output [pos] = ((c >> 10) + 0xD800) as u16 ; output [pos + 1] = ((c & 0b1111111111) + 0xDC00) as u16 ; i += 4 ; pos += 2 ; } _ => unreachable ! () , } } output }
}

macro_rules! win32_size_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function win32_size_of in module {}", module_path!());
    };
}

mkfn!{
    win32_size_of_introspect!();
    # [doc = " Helper method for getting the size of `T` as a u32."] # [doc = " Errors at compile time if the size would overflow."] # [doc = ""] # [doc = " While a type larger than u32::MAX is unlikely, it is possible if only because of a bug."] # [doc = " However, one key motivation for this function is to avoid the temptation to"] # [doc = " use frequent `as` casts. This is risky because they are too powerful."] # [doc = " For example, the following will compile today:"] # [doc = ""] # [doc = " `size_of::<u64> as u32`"] # [doc = ""] # [doc = " Note that `size_of` is never actually called, instead a function pointer is"] # [doc = " converted to a `u32`. Clippy would warn about this but, alas, it's not run"] # [doc = " on the standard library."] const fn win32_size_of < T : Sized > () -> u32 { trait Win32SizeOf : Sized { const WIN32_SIZE_OF : u32 = { let size = size_of :: < Self > () ; assert ! (size <= u32 :: MAX as usize) ; size as u32 } ; } impl < T : Sized > Win32SizeOf for T { } T :: WIN32_SIZE_OF }
}
mkitem!{mktrait!{# [doc = " The `SetFileInformationByHandle` function takes a generic parameter by"] # [doc = " making the user specify the type (class), a pointer to the data and its"] # [doc = " size. This trait allows attaching that information to a Rust type so that"] # [doc = " [`set_file_information_by_handle`] can be called safely."] # [doc = ""] # [doc = " This trait is designed so that it can support variable sized types."] # [doc = " However, currently Rust's std only uses fixed sized structures."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " * `as_ptr` must return a pointer to memory that is readable up to `size` bytes."] # [doc = " * `CLASS` must accurately reflect the type pointed to by `as_ptr`. E.g."] # [doc = " the `FILE_BASIC_INFO` structure has the class `FileBasicInfo`."] pub unsafe trait SetFileInformation { # [doc = " The type of information to set."] const CLASS : i32 ; # [doc = " A pointer to the file information to set."] fn as_ptr (& self) -> * const c_void ; # [doc = " The size of the type pointed to by `as_ptr`."] fn size (& self) -> u32 ; }}}
mkitem!{mktrait!{# [doc = " Helper trait for implementing `SetFileInformation` for statically sized types."] unsafe trait SizedSetFileInformation : Sized { const CLASS : i32 ; }}}
mkitem!{mkimpl!{unsafe impl < T : SizedSetFileInformation > SetFileInformation for T { const CLASS : i32 = T :: CLASS ; fn as_ptr (& self) -> * const c_void { (& raw const * self) . cast :: < c_void > () } fn size (& self) -> u32 { win32_size_of :: < Self > () } }}}
mkitem!{mkimpl!{unsafe impl SizedSetFileInformation for c :: FILE_BASIC_INFO { const CLASS : i32 = c :: FileBasicInfo ; }}}
mkitem!{mkimpl!{unsafe impl SizedSetFileInformation for c :: FILE_END_OF_FILE_INFO { const CLASS : i32 = c :: FileEndOfFileInfo ; }}}
mkitem!{mkimpl!{unsafe impl SizedSetFileInformation for c :: FILE_ALLOCATION_INFO { const CLASS : i32 = c :: FileAllocationInfo ; }}}
mkitem!{mkimpl!{unsafe impl SizedSetFileInformation for c :: FILE_DISPOSITION_INFO { const CLASS : i32 = c :: FileDispositionInfo ; }}}
mkitem!{mkimpl!{unsafe impl SizedSetFileInformation for c :: FILE_DISPOSITION_INFO_EX { const CLASS : i32 = c :: FileDispositionInfoEx ; }}}
mkitem!{mkimpl!{unsafe impl SizedSetFileInformation for c :: FILE_IO_PRIORITY_HINT_INFO { const CLASS : i32 = c :: FileIoPriorityHintInfo ; }}}

macro_rules! set_file_information_by_handle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_file_information_by_handle in module {}", module_path!());
    };
}

mkfn!{
    set_file_information_by_handle_introspect!();
    # [inline] pub fn set_file_information_by_handle < T : SetFileInformation > (handle : c :: HANDLE , info : & T ,) -> Result < () , WinError > { unsafe fn set_info (handle : c :: HANDLE , class : i32 , info : * const c_void , size : u32 ,) -> Result < () , WinError > { unsafe { let result = c :: SetFileInformationByHandle (handle , class , info , size) ; (result != 0) . then_some (()) . ok_or_else (get_last_error) } } unsafe { set_info (handle , T :: CLASS , info . as_ptr () , info . size ()) } }
}

macro_rules! get_last_error_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_last_error in module {}", module_path!());
    };
}

mkfn!{
    get_last_error_introspect!();
    # [doc = " Gets the error from the last function."] # [doc = " This must be called immediately after the function that sets the error to"] # [doc = " avoid the risk of another function overwriting it."] pub fn get_last_error () -> WinError { unsafe { WinError { code : c :: GetLastError () } } }
}
mkitem!{mkstruct!{# [doc = " An error code as returned by [`get_last_error`]."] # [doc = ""] # [doc = " This is usually a 16-bit Win32 error code but may be a 32-bit HRESULT or NTSTATUS."] # [doc = " Check the documentation of the Windows API function being called for expected errors."] # [derive (Clone , Copy , PartialEq , Eq)] # [repr (transparent)] pub struct WinError { pub code : u32 , }}}
mkitem!{mkimpl!{impl WinError { pub const fn new (code : u32) -> Self { Self { code } } }}}
mkitem!{mkimpl!{# [allow (unused)] impl WinError { # [doc = " Success is not an error."] # [doc = " Some Windows APIs do use this to distinguish between a zero return and an error return"] # [doc = " but we should never return this to users as an error."] pub const SUCCESS : Self = Self :: new (c :: ERROR_SUCCESS) ; pub const ACCESS_DENIED : Self = Self :: new (c :: ERROR_ACCESS_DENIED) ; pub const ALREADY_EXISTS : Self = Self :: new (c :: ERROR_ALREADY_EXISTS) ; pub const BAD_NETPATH : Self = Self :: new (c :: ERROR_BAD_NETPATH) ; pub const BAD_NET_NAME : Self = Self :: new (c :: ERROR_BAD_NET_NAME) ; pub const CANT_ACCESS_FILE : Self = Self :: new (c :: ERROR_CANT_ACCESS_FILE) ; pub const DELETE_PENDING : Self = Self :: new (c :: ERROR_DELETE_PENDING) ; pub const DIRECTORY : Self = Self :: new (c :: ERROR_DIRECTORY) ; pub const DIR_NOT_EMPTY : Self = Self :: new (c :: ERROR_DIR_NOT_EMPTY) ; pub const FILE_NOT_FOUND : Self = Self :: new (c :: ERROR_FILE_NOT_FOUND) ; pub const INSUFFICIENT_BUFFER : Self = Self :: new (c :: ERROR_INSUFFICIENT_BUFFER) ; pub const INVALID_FUNCTION : Self = Self :: new (c :: ERROR_INVALID_FUNCTION) ; pub const INVALID_HANDLE : Self = Self :: new (c :: ERROR_INVALID_HANDLE) ; pub const INVALID_PARAMETER : Self = Self :: new (c :: ERROR_INVALID_PARAMETER) ; pub const NOT_FOUND : Self = Self :: new (c :: ERROR_NOT_FOUND) ; pub const NOT_SUPPORTED : Self = Self :: new (c :: ERROR_NOT_SUPPORTED) ; pub const NO_MORE_FILES : Self = Self :: new (c :: ERROR_NO_MORE_FILES) ; pub const OPERATION_ABORTED : Self = Self :: new (c :: ERROR_OPERATION_ABORTED) ; pub const PATH_NOT_FOUND : Self = Self :: new (c :: ERROR_PATH_NOT_FOUND) ; pub const SHARING_VIOLATION : Self = Self :: new (c :: ERROR_SHARING_VIOLATION) ; pub const TIMEOUT : Self = Self :: new (c :: ERROR_TIMEOUT) ; }}}
mkitem!{mkstruct!{# [doc = " A wrapper around a UNICODE_STRING that is equivalent to `&[u16]`."] # [doc = ""] # [doc = " It is preferable to use the `unicode_str!` macro as that contains mitigations for  #143078."] # [doc = ""] # [doc = " If the MaximumLength field of the underlying UNICODE_STRING is greater than"] # [doc = " the Length field then you can test if the string is null terminated by inspecting"] # [doc = " the u16 directly after the string. You cannot otherwise depend on nul termination."] # [derive (Copy , Clone)] pub struct UnicodeStrRef < 'a > { s : c :: UNICODE_STRING , lifetime : PhantomData < & 'a [u16] > , }}}
mkitem!{static EMPTY_STRING_NULL_TERMINATED : & [u16] = & [0] ;}
mkitem!{mkimpl!{impl UnicodeStrRef < '_ > { const fn new (slice : & [u16] , is_null_terminated : bool) -> Self { let (len , max_len , ptr) = if slice . is_empty () { (0 , 2 , EMPTY_STRING_NULL_TERMINATED . as_ptr () . cast_mut ()) } else { let len = slice . len () - (is_null_terminated as usize) ; (len * 2 , size_of_val (slice) , slice . as_ptr () . cast_mut ()) } ; Self { s : c :: UNICODE_STRING { Length : len as _ , MaximumLength : max_len as _ , Buffer : ptr } , lifetime : PhantomData , } } pub const fn from_slice_with_nul (slice : & [u16]) -> Self { if ! slice . is_empty () { debug_assert ! (slice [slice . len () - 1] == 0) ; } Self :: new (slice , true) } pub const fn from_slice (slice : & [u16]) -> Self { Self :: new (slice , false) } # [doc = " Returns a pointer to the underlying UNICODE_STRING"] pub const fn as_ptr (& self) -> * const c :: UNICODE_STRING { & self . s } }}}
mkitem!{# [doc = " Create a UnicodeStringRef from a literal str or a u16 array."] # [doc = ""] # [doc = " To mitigate #143078, when using a literal str the created UNICODE_STRING"] # [doc = " will be nul terminated. The MaximumLength field of the UNICODE_STRING will"] # [doc = " be set greater than the Length field to indicate that a nul may be present."] # [doc = ""] # [doc = " If using a u16 array, the array is used exactly as provided and you cannot"] # [doc = " count on the string being nul terminated."] # [doc = " This should generally be used for strings that come from the OS."] # [doc = ""] # [doc = " **NOTE:** we lack a UNICODE_STRING builder type as we don't currently have"] # [doc = " a use for it. If needing to dynamically build a UNICODE_STRING, the builder"] # [doc = " should try to ensure there's a nul one past the end of the string."] pub macro unicode_str { ($ str : literal) => { const { crate :: sys :: pal :: windows :: api :: UnicodeStrRef :: from_slice_with_nul (crate :: sys :: pal :: windows :: api :: wide_str ! ($ str) ,) } } , ($ array : expr) => { crate :: sys :: pal :: windows :: api :: UnicodeStrRef :: from_slice ($ array ,) } }}