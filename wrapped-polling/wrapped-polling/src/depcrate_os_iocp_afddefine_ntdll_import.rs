// Generated macro for define_ntdll_import (macro)
macro_rules! Depcrate_os_iocp_afddefine_ntdll_import {
() => {
// Module: crate::os::iocp::afd
// Provides: {"define_ntdll_import"}
// Dependencies: {}
macro_rules ! define_ntdll_import { ($ ($ (# [$ attr : meta]) * fn $ name : ident ($ ($ arg : ident : $ arg_ty : ty) ,*) -> $ ret : ty ;) *) => { # [doc = " Imported functions from ntdll.dll."] # [allow (non_snake_case)] pub (super) struct NtdllImports { $ ($ (# [$ attr]) * $ name : unsafe extern "system" fn ($ ($ arg_ty) ,*) -> $ ret ,) * } # [allow (non_snake_case)] impl NtdllImports { unsafe fn load (ntdll : HMODULE) -> io :: Result < Self > { $ (# [allow (clippy :: missing_transmute_annotations)] let $ name = { const NAME : & str = concat ! (stringify ! ($ name) , "\0") ; let addr = GetProcAddress (ntdll , NAME . as_ptr () as * const _) ; let addr = match addr { Some (addr) => addr , None => { # [cfg (feature = "tracing")] tracing :: error ! ("Failed to load ntdll function {}" , NAME) ; return Err (io :: Error :: last_os_error ()) ; } , } ; transmute ::< _ , unsafe extern "system" fn ($ ($ arg_ty) ,*) -> $ ret > (addr) } ;) * Ok (Self { $ ($ name ,) * }) } $ ($ (# [$ attr]) * unsafe fn $ name (& self , $ ($ arg : $ arg_ty) ,*) -> $ ret { (self .$ name) ($ ($ arg) ,*) }) * } } ; }
};
}
