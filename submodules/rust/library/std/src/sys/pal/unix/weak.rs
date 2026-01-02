mkuse!{use crate :: ffi :: { CStr , c_char , c_void } ;}
mkuse!{use crate :: marker :: { FnPtr , PhantomData } ;}
mkuse!{use crate :: sync :: atomic :: { Atomic , AtomicPtr , Ordering } ;}
mkuse!{use crate :: { mem , ptr } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{# [cfg (all (unix , not (target_vendor = "apple")))] pub (crate) macro weak { (fn $ name : ident ($ ($ param : ident : $ t : ty) ,* $ (,) ?) -> $ ret : ty ;) => (let ref $ name : ExternWeak < unsafe extern "C" fn ($ ($ t) ,*) -> $ ret > = { unsafe extern "C" { # [linkage = "extern_weak"] static $ name : Option < unsafe extern "C" fn ($ ($ t) ,*) -> $ ret >; } # [allow (unused_unsafe)] ExternWeak :: new (unsafe { $ name }) } ;) }}
mkuse!{# [cfg (target_vendor = "apple")] pub (crate) use self :: dlsym as weak ;}
mkitem!{mkstruct!{pub (crate) struct ExternWeak < F : Copy > { weak_ptr : Option < F > , }}}
mkitem!{mkimpl!{impl < F : Copy > ExternWeak < F > { # [inline] pub (crate) fn new (weak_ptr : Option < F >) -> Self { ExternWeak { weak_ptr } } # [inline] pub (crate) fn get (& self) -> Option < F > { self . weak_ptr } }}}
mkitem!{pub (crate) macro dlsym { (fn $ name : ident ($ ($ param : ident : $ t : ty) ,* $ (,) ?) -> $ ret : ty ;) => (dlsym ! (# [link_name = stringify ! ($ name)] fn $ name ($ ($ param : $ t) ,*) -> $ ret ;) ;) , (# [link_name = $ sym : expr] fn $ name : ident ($ ($ param : ident : $ t : ty) ,* $ (,) ?) -> $ ret : ty ;) => (static DLSYM : DlsymWeak < unsafe extern "C" fn ($ ($ t) ,*) -> $ ret > = { let Ok (name) = CStr :: from_bytes_with_nul (concat ! ($ sym , '\0') . as_bytes ()) else { panic ! ("symbol name may not contain NUL") } ; unsafe { DlsymWeak :: new (name) } } ; let $ name = & DLSYM ;) }}
mkitem!{mkstruct!{pub (crate) struct DlsymWeak < F > { # [doc = " A pointer to the nul-terminated name of the symbol."] name : * const c_char , func : Atomic < * mut libc :: c_void > , _marker : PhantomData < F > , }}}
mkitem!{mkimpl!{impl < F : FnPtr > DlsymWeak < F > { # [doc = " # Safety"] # [doc = ""] # [doc = " If the signature of `F` does not match the signature of the symbol (if"] # [doc = " it exists), calling the function pointer returned by `get()` is"] # [doc = " undefined behaviour."] pub (crate) const unsafe fn new (name : & 'static CStr) -> Self { DlsymWeak { name : name . as_ptr () , func : AtomicPtr :: new (ptr :: without_provenance_mut (1)) , _marker : PhantomData , } } # [inline] pub (crate) fn get (& self) -> Option < F > { match self . func . load (Ordering :: Acquire) { func if func . addr () == 1 => self . initialize () , func if func . is_null () => None , func => Some (unsafe { mem :: transmute_copy :: < * mut c_void , F > (& func) }) , } } # [cold] fn initialize (& self) -> Option < F > { let val = unsafe { libc :: dlsym (libc :: RTLD_DEFAULT , self . name) } ; self . func . store (val , Ordering :: Release) ; if val . is_null () { None } else { Some (unsafe { mem :: transmute_copy :: < * mut libc :: c_void , F > (& val) }) } } }}}
mkitem!{mkimpl!{unsafe impl < F > Send for DlsymWeak < F > { }}}
mkitem!{mkimpl!{unsafe impl < F > Sync for DlsymWeak < F > { }}}
mkitem!{# [cfg (not (any (target_os = "linux" , target_os = "android")))] pub (crate) macro syscall { (fn $ name : ident ($ ($ param : ident : $ t : ty) ,* $ (,) ?) -> $ ret : ty ;) => (unsafe fn $ name ($ ($ param : $ t) ,*) -> $ ret { weak ! (fn $ name ($ ($ param : $ t) ,*) -> $ ret ;) ; if let Some (fun) = $ name . get () { unsafe { fun ($ ($ param) ,*) } } else { super :: os :: set_errno (libc :: ENOSYS) ; - 1 } }) }}
mkitem!{# [cfg (any (target_os = "linux" , target_os = "android"))] pub (crate) macro syscall { (fn $ name : ident ($ ($ param : ident : $ t : ty) ,* $ (,) ?) -> $ ret : ty ;) => (unsafe fn $ name ($ ($ param : $ t) ,*) -> $ ret { weak ! (fn $ name ($ ($ param : $ t) ,*) -> $ ret ;) ; if let Some (fun) = $ name . get () { unsafe { fun ($ ($ param) ,*) } } else { unsafe { libc :: syscall (libc ::$ { concat (SYS_ , $ name) } , $ ($ param) ,*) as $ ret } } }) }}
mkitem!{# [cfg (any (target_os = "linux" , target_os = "android"))] pub (crate) macro raw_syscall { (fn $ name : ident ($ ($ param : ident : $ t : ty) ,* $ (,) ?) -> $ ret : ty ;) => (unsafe fn $ name ($ ($ param : $ t) ,*) -> $ ret { unsafe { libc :: syscall (libc ::$ { concat (SYS_ , $ name) } , $ ($ param) ,*) as $ ret } }) }}