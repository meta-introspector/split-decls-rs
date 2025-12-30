// Generated macro for impl_123 (impl)
macro_rules! Depcrate_os_uniximpl_123 {
() => {
// Module: crate::os::unix
// Provides: {"impl_123"}
// Dependencies: {}
impl < T > fmt :: Debug for Symbol < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { unsafe { let mut info = mem :: MaybeUninit :: < DlInfo > :: uninit () ; if dladdr (self . pointer , info . as_mut_ptr ()) != 0 { let info = info . assume_init () ; if info . dli_sname . is_null () { f . write_fmt (format_args ! ("Symbol@{:p} from {:?}" , self . pointer , CStr :: from_ptr (info . dli_fname))) } else { f . write_fmt (format_args ! ("Symbol {:?}@{:p} from {:?}" , CStr :: from_ptr (info . dli_sname) , self . pointer , CStr :: from_ptr (info . dli_fname))) } } else { f . write_fmt (format_args ! ("Symbol@{:p}" , self . pointer)) } } } }
};
}
