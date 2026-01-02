mkuse!{use core :: marker :: PhantomData ;}
mkuse!{use core :: num :: NonZeroUsize ;}
mkuse!{use core :: ptr :: NonNull ;}
mkuse!{use super :: { Custom , ErrorData , ErrorKind , RawOsError , SimpleMessage } ;}
mkitem!{const TAG_MASK : usize = 0b11 ;}
mkitem!{const TAG_SIMPLE_MESSAGE : usize = 0b00 ;}
mkitem!{const TAG_CUSTOM : usize = 0b01 ;}
mkitem!{const TAG_OS : usize = 0b10 ;}
mkitem!{const TAG_SIMPLE : usize = 0b11 ;}
mkitem!{mkstruct!{# [doc = " The internal representation."] # [doc = ""] # [doc = " See the module docs for more, this is just a way to hack in a check that we"] # [doc = " indeed are not unwind-safe."] # [doc = ""] # [doc = " ```compile_fail,E0277"] # [doc = " fn is_unwind_safe<T: core::panic::UnwindSafe>() {}"] # [doc = " is_unwind_safe::<std::io::Error>();"] # [doc = " ```"] # [repr (transparent)] # [rustc_insignificant_dtor] pub (super) struct Repr (NonNull < () > , PhantomData < ErrorData < Box < Custom > > >) ;}}
mkitem!{mkimpl!{unsafe impl Send for Repr { }}}
mkitem!{mkimpl!{unsafe impl Sync for Repr { }}}
mkitem!{mkimpl!{impl Repr { pub (super) fn new (dat : ErrorData < Box < Custom > >) -> Self { match dat { ErrorData :: Os (code) => Self :: new_os (code) , ErrorData :: Simple (kind) => Self :: new_simple (kind) , ErrorData :: SimpleMessage (simple_message) => Self :: new_simple_message (simple_message) , ErrorData :: Custom (b) => Self :: new_custom (b) , } } pub (super) fn new_custom (b : Box < Custom >) -> Self { let p = Box :: into_raw (b) . cast :: < u8 > () ; debug_assert_eq ! (p . addr () & TAG_MASK , 0) ; let tagged = p . wrapping_add (TAG_CUSTOM) . cast :: < () > () ; let res = Self (unsafe { NonNull :: new_unchecked (tagged) } , PhantomData) ; debug_assert ! (matches ! (res . data () , ErrorData :: Custom (_)) , "repr(custom) encoding failed") ; res } # [inline] pub (super) fn new_os (code : RawOsError) -> Self { let utagged = ((code as usize) << 32) | TAG_OS ; let res = Self (NonNull :: without_provenance (unsafe { NonZeroUsize :: new_unchecked (utagged) }) , PhantomData ,) ; debug_assert ! (matches ! (res . data () , ErrorData :: Os (c) if c == code) , "repr(os) encoding failed for {code}") ; res } # [inline] pub (super) fn new_simple (kind : ErrorKind) -> Self { let utagged = ((kind as usize) << 32) | TAG_SIMPLE ; let res = Self (NonNull :: without_provenance (unsafe { NonZeroUsize :: new_unchecked (utagged) }) , PhantomData ,) ; debug_assert ! (matches ! (res . data () , ErrorData :: Simple (k) if k == kind) , "repr(simple) encoding failed {:?}" , kind ,) ; res } # [inline] pub (super) const fn new_simple_message (m : & 'static SimpleMessage) -> Self { Self (unsafe { NonNull :: new_unchecked (m as * const _ as * mut ()) } , PhantomData) } # [inline] pub (super) fn data (& self) -> ErrorData < & Custom > { unsafe { decode_repr (self . 0 , | c | & * c) } } # [inline] pub (super) fn data_mut (& mut self) -> ErrorData < & mut Custom > { unsafe { decode_repr (self . 0 , | c | & mut * c) } } # [inline] pub (super) fn into_data (self) -> ErrorData < Box < Custom > > { let this = core :: mem :: ManuallyDrop :: new (self) ; unsafe { decode_repr (this . 0 , | p | Box :: from_raw (p)) } } }}}
mkitem!{mkimpl!{impl Drop for Repr { # [inline] fn drop (& mut self) { unsafe { let _ = decode_repr (self . 0 , | p | Box :: < Custom > :: from_raw (p)) ; } } }}}

macro_rules! decode_repr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function decode_repr in module {}", module_path!());
    };
}

mkfn!{
    decode_repr_introspect!();
    # [inline] unsafe fn decode_repr < C , F > (ptr : NonNull < () > , make_custom : F) -> ErrorData < C > where F : FnOnce (* mut Custom) -> C , { let bits = ptr . as_ptr () . addr () ; match bits & TAG_MASK { TAG_OS => { let code = ((bits as i64) >> 32) as RawOsError ; ErrorData :: Os (code) } TAG_SIMPLE => { let kind_bits = (bits >> 32) as u32 ; let kind = kind_from_prim (kind_bits) . unwrap_or_else (| | { debug_assert ! (false , "Invalid io::error::Repr bits: `Repr({:#018x})`" , bits) ; unsafe { core :: hint :: unreachable_unchecked () } ; }) ; ErrorData :: Simple (kind) } TAG_SIMPLE_MESSAGE => { unsafe { ErrorData :: SimpleMessage (& * ptr . cast :: < SimpleMessage > () . as_ptr ()) } } TAG_CUSTOM => { let custom = ptr . as_ptr () . wrapping_byte_sub (TAG_CUSTOM) . cast :: < Custom > () ; ErrorData :: Custom (make_custom (custom)) } _ => { unreachable ! () ; } } }
}

macro_rules! kind_from_prim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function kind_from_prim in module {}", module_path!());
    };
}

mkfn!{
    kind_from_prim_introspect!();
    # [inline] fn kind_from_prim (ek : u32) -> Option < ErrorKind > { macro_rules ! from_prim { ($ prim : expr => $ Enum : ident { $ ($ Variant : ident) ,* $ (,) ? }) => { { const _ : fn (e : $ Enum) = | e : $ Enum | match e { $ ($ Enum ::$ Variant => ()) ,* } ; match $ prim { $ (v if v == ($ Enum ::$ Variant as _) => Some ($ Enum ::$ Variant) ,) * _ => None , } } } } from_prim ! (ek => ErrorKind { NotFound , PermissionDenied , ConnectionRefused , ConnectionReset , HostUnreachable , NetworkUnreachable , ConnectionAborted , NotConnected , AddrInUse , AddrNotAvailable , NetworkDown , BrokenPipe , AlreadyExists , WouldBlock , NotADirectory , IsADirectory , DirectoryNotEmpty , ReadOnlyFilesystem , FilesystemLoop , StaleNetworkFileHandle , InvalidInput , InvalidData , TimedOut , WriteZero , StorageFull , NotSeekable , QuotaExceeded , FileTooLarge , ResourceBusy , ExecutableFileBusy , Deadlock , CrossesDevices , TooManyLinks , InvalidFilename , ArgumentListTooLong , Interrupted , Other , UnexpectedEof , Unsupported , OutOfMemory , InProgress , Uncategorized , }) }
}
mkitem!{macro_rules ! static_assert { ($ condition : expr) => { const _ : () = assert ! ($ condition) ; } ; (@ usize_eq : $ lhs : expr , $ rhs : expr) => { const _ : [() ; $ lhs] = [() ; $ rhs] ; } ; }}
mkitem!{static_assert ! (@ usize_eq : size_of ::< NonNull < () >> () , 8) ;}
mkitem!{static_assert ! (@ usize_eq : size_of ::< NonNull < () >> () , size_of ::< usize > ()) ;}
mkitem!{static_assert ! (@ usize_eq : size_of ::<&'static SimpleMessage > () , 8) ;}
mkitem!{static_assert ! (@ usize_eq : size_of ::< Box < Custom >> () , 8) ;}
mkitem!{static_assert ! ((TAG_MASK + 1) . is_power_of_two ()) ;}
mkitem!{static_assert ! (align_of ::< SimpleMessage > () >= TAG_MASK + 1) ;}
mkitem!{static_assert ! (align_of ::< Custom > () >= TAG_MASK + 1) ;}
mkitem!{static_assert ! (@ usize_eq : TAG_MASK & TAG_SIMPLE_MESSAGE , TAG_SIMPLE_MESSAGE) ;}
mkitem!{static_assert ! (@ usize_eq : TAG_MASK & TAG_CUSTOM , TAG_CUSTOM) ;}
mkitem!{static_assert ! (@ usize_eq : TAG_MASK & TAG_OS , TAG_OS) ;}
mkitem!{static_assert ! (@ usize_eq : TAG_MASK & TAG_SIMPLE , TAG_SIMPLE) ;}
mkitem!{static_assert ! (size_of ::< Custom > () >= TAG_CUSTOM) ;}
mkitem!{static_assert ! (TAG_OS != 0) ;}
mkitem!{static_assert ! (TAG_SIMPLE != 0) ;}
mkitem!{static_assert ! (@ usize_eq : TAG_SIMPLE_MESSAGE , 0) ;}
mkitem!{static_assert ! (@ usize_eq : size_of ::< Repr > () , 8) ;}
mkitem!{static_assert ! (@ usize_eq : size_of ::< Option < Repr >> () , 8) ;}
mkitem!{static_assert ! (@ usize_eq : size_of ::< Result < () , Repr >> () , 8) ;}
mkitem!{static_assert ! (@ usize_eq : size_of ::< Result < usize , Repr >> () , 16) ;}