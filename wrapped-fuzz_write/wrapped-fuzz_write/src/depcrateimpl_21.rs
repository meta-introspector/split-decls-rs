// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl FuzzTestCase < '_ > { fn execute (self , stringifier : & mut impl Write , panic_on_error : bool) -> ZipResult < () > { let mut initial_junk = Cursor :: new (self . initial_junk . into_vec ()) ; initial_junk . seek (SeekFrom :: End (0)) ? ; let mut writer = zip :: ZipWriter :: new (initial_junk) ; let mut files_added = 0 ; let mut final_reopen = false ; if let Some ((last_op , _)) = self . operations . last () { if last_op . reopen != ReopenOption :: ViaFinishIntoReadable { final_reopen = true ; } } # [allow (unknown_lints)] # [allow (boxed_slice_into_iter)] for (operation , abort) in self . operations . into_vec () . into_iter () { let _ = do_operation (& mut writer , operation , abort , self . flush_on_finish_file , & mut files_added , stringifier , panic_on_error ,) ; } if final_reopen { writeln ! (stringifier , "let _ = writer.finish_into_readable()?;") . map_err (| _ | ZipError :: InvalidArchive ("" . into ())) ? ; let _ = writer . finish_into_readable () ? ; } Ok (()) } }
};
}
