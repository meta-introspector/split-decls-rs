mkuse!{use crate :: sys :: pal :: itron :: abi ;}
mkuse!{use crate :: sys :: pal :: itron :: error :: { ItronError , expect_success , expect_success_aborting , fail } ;}
mkuse!{use crate :: sys :: pal :: itron :: spin :: SpinIdOnceCell ;}
mkitem!{mkstruct!{pub struct Mutex { # [doc = " The ID of the underlying mutex object"] mtx : SpinIdOnceCell < () > , }}}

macro_rules! new_mtx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_mtx in module {}", module_path!());
    };
}

mkfn!{
    new_mtx_introspect!();
    # [doc = " Creates a mutex object. This function never panics."] fn new_mtx () -> Result < abi :: ID , ItronError > { ItronError :: err_if_negative (unsafe { abi :: acre_mtx (& abi :: T_CMTX { mtxatr : abi :: TA_INHERIT , ceilpri : 0 , }) }) }
}
mkitem!{mkimpl!{impl Mutex { # [inline] pub const fn new () -> Mutex { Mutex { mtx : SpinIdOnceCell :: new () } } # [doc = " Gets the inner mutex's ID, which is lazily created."] fn raw (& self) -> abi :: ID { match self . mtx . get_or_try_init (| | new_mtx () . map (| id | (id , ()))) { Ok ((id , ())) => id , Err (e) => fail (e , & "acre_mtx") , } } pub fn lock (& self) { let mtx = self . raw () ; expect_success (unsafe { abi :: loc_mtx (mtx) } , & "loc_mtx") ; } pub unsafe fn unlock (& self) { let mtx = unsafe { self . mtx . get_unchecked () . 0 } ; expect_success_aborting (unsafe { abi :: unl_mtx (mtx) } , & "unl_mtx") ; } pub fn try_lock (& self) -> bool { let mtx = self . raw () ; match unsafe { abi :: ploc_mtx (mtx) } { abi :: E_TMOUT => false , er => { expect_success (er , & "ploc_mtx") ; true } } } }}}
mkitem!{mkimpl!{impl Drop for Mutex { fn drop (& mut self) { if let Some (mtx) = self . mtx . get () . map (| x | x . 0) { expect_success_aborting (unsafe { abi :: del_mtx (mtx) } , & "del_mtx") ; } } }}}