mkuse!{use crate :: io ;}
mkuse!{use crate :: iter :: Iterator ;}
mkuse!{use crate :: mem :: MaybeUninit ;}
mkuse!{use crate :: os :: uefi ;}
mkuse!{use crate :: ptr :: NonNull ;}
mkitem!{mkstruct!{pub struct Stdin { surrogate : Option < u16 > , incomplete_utf8 : IncompleteUtf8 , }}}
mkitem!{mkstruct!{struct IncompleteUtf8 { bytes : [u8 ; 4] , len : u8 , }}}
mkitem!{mkimpl!{impl IncompleteUtf8 { pub const fn new () -> IncompleteUtf8 { IncompleteUtf8 { bytes : [0 ; 4] , len : 0 } } fn read (& mut self , buf : & mut [u8]) -> usize { let to_write = crate :: cmp :: min (buf . len () , self . len as usize) ; buf [.. to_write] . copy_from_slice (& self . bytes [.. to_write]) ; if usize :: from (self . len) > buf . len () { self . bytes . copy_within (to_write .. , 0) ; self . len -= to_write as u8 ; } else { self . len = 0 ; } to_write } }}}
mkitem!{mkstruct!{pub struct Stdout ;}}
mkitem!{mkstruct!{pub struct Stderr ;}}
mkitem!{mkimpl!{impl Stdin { pub const fn new () -> Stdin { Stdin { surrogate : None , incomplete_utf8 : IncompleteUtf8 :: new () } } }}}
mkitem!{mkimpl!{impl io :: Read for Stdin { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let mut bytes_copied = self . incomplete_utf8 . read (buf) ; let stdin : * mut r_efi :: protocols :: simple_text_input :: Protocol = unsafe { let st : NonNull < r_efi :: efi :: SystemTable > = uefi :: env :: system_table () . cast () ; (* st . as_ptr ()) . con_in } ; if bytes_copied == buf . len () { return Ok (bytes_copied) ; } let ch = simple_text_input_read (stdin) ? ; let mut ch : Vec < Result < char , crate :: char :: DecodeUtf16Error > > = if let Some (x) = self . surrogate . take () { char :: decode_utf16 ([x , ch]) . collect () } else { char :: decode_utf16 ([ch]) . collect () } ; if ch . len () > 1 { return Err (io :: const_error ! (io :: ErrorKind :: InvalidData , "invalid UTF-16 sequence")) ; } match ch . pop () . unwrap () { Err (e) => { self . surrogate = Some (e . unpaired_surrogate ()) ; } Ok (x) => { let buf_free_count = buf . len () - bytes_copied ; assert ! (buf_free_count > 0) ; if buf_free_count >= x . len_utf8 () { bytes_copied += x . encode_utf8 (& mut buf [bytes_copied ..]) . len () ; } else { self . incomplete_utf8 . len = x . encode_utf8 (& mut self . incomplete_utf8 . bytes) . len () as u8 ; bytes_copied += self . incomplete_utf8 . read (buf) ; } } } Ok (bytes_copied) } }}}
mkitem!{mkimpl!{impl Stdout { pub const fn new () -> Stdout { Stdout } }}}
mkitem!{mkimpl!{impl io :: Write for Stdout { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let st : NonNull < r_efi :: efi :: SystemTable > = uefi :: env :: system_table () . cast () ; let stdout = unsafe { (* st . as_ptr ()) . con_out } ; write (stdout , buf) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{mkimpl!{impl Stderr { pub const fn new () -> Stderr { Stderr } }}}
mkitem!{mkimpl!{impl io :: Write for Stderr { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let st : NonNull < r_efi :: efi :: SystemTable > = uefi :: env :: system_table () . cast () ; let stderr = unsafe { (* st . as_ptr ()) . std_err } ; write (stderr , buf) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }}}
mkitem!{pub const STDIN_BUF_SIZE : usize = 4 ;}

macro_rules! is_ebadf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_ebadf in module {}", module_path!());
    };
}

mkfn!{
    is_ebadf_introspect!();
    pub fn is_ebadf (err : & io :: Error) -> bool { if let Some (x) = err . raw_os_error () { r_efi :: efi :: Status :: UNSUPPORTED . as_usize () == x } else { false } }
}

macro_rules! panic_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_output in module {}", module_path!());
    };
}

mkfn!{
    panic_output_introspect!();
    pub fn panic_output () -> Option < impl io :: Write > { uefi :: env :: try_system_table () . map (| _ | Stderr :: new ()) }
}

macro_rules! write_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write in module {}", module_path!());
    };
}

mkfn!{
    write_introspect!();
    fn write (protocol : * mut r_efi :: protocols :: simple_text_output :: Protocol , buf : & [u8] ,) -> io :: Result < usize > { let utf8 = match crate :: str :: from_utf8 (buf) { Ok (x) => x , Err (e) => unsafe { crate :: str :: from_utf8_unchecked (& buf [.. e . valid_up_to ()]) } , } ; let mut utf16 : Vec < u16 > = utf8 . encode_utf16 () . collect () ; utf16 . push (0) ; unsafe { simple_text_output (protocol , & mut utf16) } ? ; Ok (utf8 . len ()) }
}

macro_rules! simple_text_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function simple_text_output in module {}", module_path!());
    };
}

mkfn!{
    simple_text_output_introspect!();
    unsafe fn simple_text_output (protocol : * mut r_efi :: protocols :: simple_text_output :: Protocol , buf : & mut [u16] ,) -> io :: Result < () > { let res = unsafe { ((* protocol) . output_string) (protocol , buf . as_mut_ptr ()) } ; if res . is_error () { Err (io :: Error :: from_raw_os_error (res . as_usize ())) } else { Ok (()) } }
}

macro_rules! simple_text_input_read_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function simple_text_input_read in module {}", module_path!());
    };
}

mkfn!{
    simple_text_input_read_introspect!();
    fn simple_text_input_read (stdin : * mut r_efi :: protocols :: simple_text_input :: Protocol ,) -> io :: Result < u16 > { loop { match read_key_stroke (stdin) { Ok (x) => return Ok (x . unicode_char) , Err (e) if e == r_efi :: efi :: Status :: NOT_READY => wait_stdin (stdin) ? , Err (e) => return Err (io :: Error :: from_raw_os_error (e . as_usize ())) , } } }
}

macro_rules! wait_stdin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function wait_stdin in module {}", module_path!());
    };
}

mkfn!{
    wait_stdin_introspect!();
    fn wait_stdin (stdin : * mut r_efi :: protocols :: simple_text_input :: Protocol) -> io :: Result < () > { let boot_services : NonNull < r_efi :: efi :: BootServices > = uefi :: env :: boot_services () . unwrap () . cast () ; let wait_for_event = unsafe { (* boot_services . as_ptr ()) . wait_for_event } ; let wait_for_key_event = unsafe { (* stdin) . wait_for_key } ; let r = { let mut x : usize = 0 ; (wait_for_event) (1 , [wait_for_key_event] . as_mut_ptr () , & mut x) } ; if r . is_error () { Err (io :: Error :: from_raw_os_error (r . as_usize ())) } else { Ok (()) } }
}

macro_rules! read_key_stroke_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_key_stroke in module {}", module_path!());
    };
}

mkfn!{
    read_key_stroke_introspect!();
    fn read_key_stroke (stdin : * mut r_efi :: protocols :: simple_text_input :: Protocol ,) -> Result < r_efi :: protocols :: simple_text_input :: InputKey , r_efi :: efi :: Status > { let mut input_key : MaybeUninit < r_efi :: protocols :: simple_text_input :: InputKey > = MaybeUninit :: uninit () ; let r = unsafe { ((* stdin) . read_key_stroke) (stdin , input_key . as_mut_ptr ()) } ; if r . is_error () { Err (r) } else { let input_key = unsafe { input_key . assume_init () } ; Ok (input_key) } }
}