mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkuse!{use crate :: backtrace_rs :: { self , BytesOrWideString } ;}
mkuse!{use crate :: ffi :: c_void ;}
mkuse!{use crate :: panic :: UnwindSafe ;}
mkuse!{use crate :: sync :: LazyLock ;}
mkuse!{use crate :: sync :: atomic :: Ordering :: Relaxed ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicU8 } ;}
mkuse!{use crate :: sys :: backtrace :: { lock , output_filename , set_image_base } ;}
mkuse!{use crate :: { env , fmt } ;}
mkitem!{mkstruct!{# [doc = " A captured OS thread stack backtrace."] # [doc = ""] # [doc = " This type represents a stack backtrace for an OS thread captured at a"] # [doc = " previous point in time. In some instances the `Backtrace` type may"] # [doc = " internally be empty due to configuration. For more information see"] # [doc = " `Backtrace::capture`."] # [stable (feature = "backtrace" , since = "1.65.0")] # [must_use] pub struct Backtrace { inner : Inner , }}}
mkitem!{mkenum!{# [doc = " The current status of a backtrace, indicating whether it was captured or"] # [doc = " whether it is empty for some other reason."] # [stable (feature = "backtrace" , since = "1.65.0")] # [non_exhaustive] # [derive (Debug , PartialEq , Eq)] pub enum BacktraceStatus { # [doc = " Capturing a backtrace is not supported, likely because it's not"] # [doc = " implemented for the current platform."] # [stable (feature = "backtrace" , since = "1.65.0")] Unsupported , # [doc = " Capturing a backtrace has been disabled through either the"] # [doc = " `RUST_LIB_BACKTRACE` or `RUST_BACKTRACE` environment variables."] # [stable (feature = "backtrace" , since = "1.65.0")] Disabled , # [doc = " A backtrace has been captured and the `Backtrace` should print"] # [doc = " reasonable information when rendered."] # [stable (feature = "backtrace" , since = "1.65.0")] Captured , }}}
mkitem!{mkenum!{enum Inner { Unsupported , Disabled , Captured (LazyLock < Capture , LazyResolve >) , }}}
mkitem!{mkstruct!{struct Capture { actual_start : usize , frames : Vec < BacktraceFrame > , }}}

macro_rules! _assert_send_sync_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _assert_send_sync in module {}", module_path!());
    };
}

mkfn!{
    _assert_send_sync_introspect!();
    fn _assert_send_sync () { fn _assert < T : Send + Sync > () { } _assert :: < Backtrace > () ; }
}
mkitem!{mkstruct!{# [doc = " A single frame of a backtrace."] # [unstable (feature = "backtrace_frames" , issue = "79676")] pub struct BacktraceFrame { frame : RawFrame , symbols : Vec < BacktraceSymbol > , }}}
mkitem!{mkenum!{# [derive (Debug)] enum RawFrame { Actual (backtrace_rs :: Frame) , # [cfg (test)] Fake , }}}
mkitem!{mkstruct!{struct BacktraceSymbol { name : Option < Vec < u8 > > , filename : Option < BytesOrWide > , lineno : Option < u32 > , colno : Option < u32 > , }}}
mkitem!{mkenum!{enum BytesOrWide { Bytes (Vec < u8 >) , Wide (Vec < u16 >) , }}}
mkitem!{mkimpl!{# [stable (feature = "backtrace" , since = "1.65.0")] impl fmt :: Debug for Backtrace { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let capture = match & self . inner { Inner :: Unsupported => return fmt . write_str ("<unsupported>") , Inner :: Disabled => return fmt . write_str ("<disabled>") , Inner :: Captured (c) => & * * c , } ; let frames = & capture . frames [capture . actual_start ..] ; write ! (fmt , "Backtrace ") ? ; let mut dbg = fmt . debug_list () ; for frame in frames { if frame . frame . ip () . is_null () { continue ; } dbg . entries (& frame . symbols) ; } dbg . finish () } }}}
mkitem!{mkimpl!{# [unstable (feature = "backtrace_frames" , issue = "79676")] impl fmt :: Debug for BacktraceFrame { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut dbg = fmt . debug_list () ; dbg . entries (& self . symbols) ; dbg . finish () } }}}
mkitem!{mkimpl!{impl fmt :: Debug for BacktraceSymbol { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "{{ ") ? ; if let Some (fn_name) = self . name . as_ref () . map (| b | backtrace_rs :: SymbolName :: new (b)) { write ! (fmt , "fn: \"{:#}\"" , fn_name) ? ; } else { write ! (fmt , "fn: <unknown>") ? ; } if let Some (fname) = self . filename . as_ref () { write ! (fmt , ", file: \"{:?}\"" , fname) ? ; } if let Some (line) = self . lineno { write ! (fmt , ", line: {:?}" , line) ? ; } write ! (fmt , " }}") } }}}
mkitem!{mkimpl!{impl fmt :: Debug for BytesOrWide { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { output_filename (fmt , match self { BytesOrWide :: Bytes (w) => BytesOrWideString :: Bytes (w) , BytesOrWide :: Wide (w) => BytesOrWideString :: Wide (w) , } , backtrace_rs :: PrintFmt :: Short , crate :: env :: current_dir () . as_ref () . ok () ,) } }}}
mkitem!{mkimpl!{impl Backtrace { # [doc = " Returns whether backtrace captures are enabled through environment"] # [doc = " variables."] fn enabled () -> bool { static ENABLED : Atomic < u8 > = AtomicU8 :: new (0) ; match ENABLED . load (Relaxed) { 0 => { } 1 => return false , _ => return true , } let enabled = match env :: var ("RUST_LIB_BACKTRACE") { Ok (s) => s != "0" , Err (_) => match env :: var ("RUST_BACKTRACE") { Ok (s) => s != "0" , Err (_) => false , } , } ; ENABLED . store (enabled as u8 + 1 , Relaxed) ; enabled } # [doc = " Captures a stack backtrace of the current thread."] # [doc = ""] # [doc = " This function will capture a stack backtrace of the current OS thread of"] # [doc = " execution, returning a `Backtrace` type which can be later used to print"] # [doc = " the entire stack trace or render it to a string."] # [doc = ""] # [doc = " This function will be a noop if the `RUST_BACKTRACE` or"] # [doc = " `RUST_LIB_BACKTRACE` backtrace variables are both not set. If either"] # [doc = " environment variable is set and enabled then this function will actually"] # [doc = " capture a backtrace. Capturing a backtrace can be both memory intensive"] # [doc = " and slow, so these environment variables allow liberally using"] # [doc = " `Backtrace::capture` and only incurring a slowdown when the environment"] # [doc = " variables are set."] # [doc = ""] # [doc = " To forcibly capture a backtrace regardless of environment variables, use"] # [doc = " the `Backtrace::force_capture` function."] # [stable (feature = "backtrace" , since = "1.65.0")] # [inline (never)] pub fn capture () -> Backtrace { if ! Backtrace :: enabled () { return Backtrace { inner : Inner :: Disabled } ; } Backtrace :: create (Backtrace :: capture as usize) } # [doc = " Forcibly captures a full backtrace, regardless of environment variable"] # [doc = " configuration."] # [doc = ""] # [doc = " This function behaves the same as `capture` except that it ignores the"] # [doc = " values of the `RUST_BACKTRACE` and `RUST_LIB_BACKTRACE` environment"] # [doc = " variables, always capturing a backtrace."] # [doc = ""] # [doc = " Note that capturing a backtrace can be an expensive operation on some"] # [doc = " platforms, so this should be used with caution in performance-sensitive"] # [doc = " parts of code."] # [stable (feature = "backtrace" , since = "1.65.0")] # [inline (never)] pub fn force_capture () -> Backtrace { Backtrace :: create (Backtrace :: force_capture as usize) } # [doc = " Forcibly captures a disabled backtrace, regardless of environment"] # [doc = " variable configuration."] # [stable (feature = "backtrace" , since = "1.65.0")] # [rustc_const_stable (feature = "backtrace" , since = "1.65.0")] pub const fn disabled () -> Backtrace { Backtrace { inner : Inner :: Disabled } } fn create (ip : usize) -> Backtrace { let _lock = lock () ; let mut frames = Vec :: new () ; let mut actual_start = None ; set_image_base () ; unsafe { backtrace_rs :: trace_unsynchronized (| frame | { frames . push (BacktraceFrame { frame : RawFrame :: Actual (frame . clone ()) , symbols : Vec :: new () , }) ; if frame . symbol_address () . addr () == ip && actual_start . is_none () { actual_start = Some (frames . len ()) ; } true }) ; } let inner = if frames . is_empty () { Inner :: Unsupported } else { Inner :: Captured (LazyLock :: new (lazy_resolve (Capture { actual_start : actual_start . unwrap_or (0) , frames , }))) } ; Backtrace { inner } } # [doc = " Returns the status of this backtrace, indicating whether this backtrace"] # [doc = " request was unsupported, disabled, or a stack trace was actually"] # [doc = " captured."] # [stable (feature = "backtrace" , since = "1.65.0")] # [must_use] pub fn status (& self) -> BacktraceStatus { match self . inner { Inner :: Unsupported => BacktraceStatus :: Unsupported , Inner :: Disabled => BacktraceStatus :: Disabled , Inner :: Captured (_) => BacktraceStatus :: Captured , } } }}}
mkitem!{mkimpl!{impl < 'a > Backtrace { # [doc = " Returns an iterator over the backtrace frames."] # [must_use] # [unstable (feature = "backtrace_frames" , issue = "79676")] pub fn frames (& 'a self) -> & 'a [BacktraceFrame] { if let Inner :: Captured (c) = & self . inner { & c . frames } else { & [] } } }}}
mkitem!{mkimpl!{# [stable (feature = "backtrace" , since = "1.65.0")] impl fmt :: Display for Backtrace { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let capture = match & self . inner { Inner :: Unsupported => return fmt . write_str ("unsupported backtrace") , Inner :: Disabled => return fmt . write_str ("disabled backtrace") , Inner :: Captured (c) => & * * c , } ; let full = fmt . alternate () ; let (frames , style) = if full { (& capture . frames [..] , backtrace_rs :: PrintFmt :: Full) } else { (& capture . frames [capture . actual_start ..] , backtrace_rs :: PrintFmt :: Short) } ; let cwd = crate :: env :: current_dir () ; let mut print_path = move | fmt : & mut fmt :: Formatter < '_ > , path : BytesOrWideString < '_ > | { output_filename (fmt , path , style , cwd . as_ref () . ok ()) } ; let mut f = backtrace_rs :: BacktraceFmt :: new (fmt , style , & mut print_path) ; f . add_context () ? ; for frame in frames { if frame . symbols . is_empty () { f . frame () . print_raw (frame . frame . ip () , None , None , None) ? ; } else { for symbol in frame . symbols . iter () { f . frame () . print_raw_with_column (frame . frame . ip () , symbol . name . as_ref () . map (| b | backtrace_rs :: SymbolName :: new (b)) , symbol . filename . as_ref () . map (| b | match b { BytesOrWide :: Bytes (w) => BytesOrWideString :: Bytes (w) , BytesOrWide :: Wide (w) => BytesOrWideString :: Wide (w) , }) , symbol . lineno , symbol . colno ,) ? ; } } } f . finish () ? ; Ok (()) } }}}
mkmod!{helper, { 
                getname!(helper);
                getsrc!(helper);
                getpath!(helper);
                get_deps!(helper);
                get_crates!(helper);
                mkinclude!(helper);
                mkuse!{use super :: * ;}
mkitem!{pub (super) type LazyResolve = impl (FnOnce () -> Capture) + Send + Sync + UnwindSafe ;}

macro_rules! lazy_resolve_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lazy_resolve in module {}", module_path!());
    };
}

mkfn!{
    lazy_resolve_introspect!();
    # [define_opaque (LazyResolve)] pub (super) fn lazy_resolve (mut capture : Capture) -> LazyResolve { move | | { let _lock = lock () ; for frame in capture . frames . iter_mut () { let symbols = & mut frame . symbols ; let frame = match & frame . frame { RawFrame :: Actual (frame) => frame , # [cfg (test)] RawFrame :: Fake => unimplemented ! () , } ; unsafe { backtrace_rs :: resolve_frame_unsynchronized (frame , | symbol | { symbols . push (BacktraceSymbol { name : symbol . name () . map (| m | m . as_bytes () . to_vec ()) , filename : symbol . filename_raw () . map (| b | match b { BytesOrWideString :: Bytes (b) => BytesOrWide :: Bytes (b . to_owned ()) , BytesOrWideString :: Wide (b) => BytesOrWide :: Wide (b . to_owned ()) , }) , lineno : symbol . lineno () , colno : symbol . colno () , }) ; }) ; } } capture } }
} 
            }}
mkuse!{use helper :: * ;}
mkitem!{mkimpl!{impl RawFrame { fn ip (& self) -> * mut c_void { match self { RawFrame :: Actual (frame) => frame . ip () , # [cfg (test)] RawFrame :: Fake => crate :: ptr :: without_provenance_mut (1) , } } }}}