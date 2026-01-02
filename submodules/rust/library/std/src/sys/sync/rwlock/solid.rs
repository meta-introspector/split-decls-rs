mkuse!{use crate :: sys :: pal :: abi ;}
mkuse!{use crate :: sys :: pal :: itron :: error :: { ItronError , expect_success , expect_success_aborting , fail } ;}
mkuse!{use crate :: sys :: pal :: itron :: spin :: SpinIdOnceCell ;}
mkitem!{mkstruct!{pub struct RwLock { # [doc = " The ID of the underlying mutex object"] rwl : SpinIdOnceCell < () > , }}}
mkitem!{mkimpl!{unsafe impl Send for RwLock { }}}
mkitem!{mkimpl!{unsafe impl Sync for RwLock { }}}

macro_rules! new_rwl_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function new_rwl in module {}", module_path!());
    };
}

mkfn!{
    new_rwl_introspect!();
    fn new_rwl () -> Result < abi :: ID , ItronError > { ItronError :: err_if_negative (unsafe { abi :: rwl_acre_rwl () }) }
}
mkitem!{mkimpl!{impl RwLock { # [inline] pub const fn new () -> RwLock { RwLock { rwl : SpinIdOnceCell :: new () } } # [doc = " Gets the inner mutex's ID, which is lazily created."] fn raw (& self) -> abi :: ID { match self . rwl . get_or_try_init (| | new_rwl () . map (| id | (id , ()))) { Ok ((id , ())) => id , Err (e) => fail (e , & "rwl_acre_rwl") , } } # [inline] pub fn read (& self) { let rwl = self . raw () ; expect_success (unsafe { abi :: rwl_loc_rdl (rwl) } , & "rwl_loc_rdl") ; } # [inline] pub fn try_read (& self) -> bool { let rwl = self . raw () ; match unsafe { abi :: rwl_ploc_rdl (rwl) } { abi :: E_TMOUT => false , er => { expect_success (er , & "rwl_ploc_rdl") ; true } } } # [inline] pub fn write (& self) { let rwl = self . raw () ; expect_success (unsafe { abi :: rwl_loc_wrl (rwl) } , & "rwl_loc_wrl") ; } # [inline] pub fn try_write (& self) -> bool { let rwl = self . raw () ; match unsafe { abi :: rwl_ploc_wrl (rwl) } { abi :: E_TMOUT => false , er => { expect_success (er , & "rwl_ploc_wrl") ; true } } } # [inline] pub unsafe fn read_unlock (& self) { let rwl = self . raw () ; expect_success_aborting (unsafe { abi :: rwl_unl_rwl (rwl) } , & "rwl_unl_rwl") ; } # [inline] pub unsafe fn write_unlock (& self) { let rwl = self . raw () ; expect_success_aborting (unsafe { abi :: rwl_unl_rwl (rwl) } , & "rwl_unl_rwl") ; } # [inline] pub unsafe fn downgrade (& self) { } }}}
mkitem!{mkimpl!{impl Drop for RwLock { # [inline] fn drop (& mut self) { if let Some (rwl) = self . rwl . get () . map (| x | x . 0) { expect_success_aborting (unsafe { abi :: rwl_del_rwl (rwl) } , & "rwl_del_rwl") ; } } }}}