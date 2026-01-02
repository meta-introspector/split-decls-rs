mkuse!{use std :: mem :: ManuallyDrop ;}
mkuse!{use std :: path :: Path ;}
mkuse!{use tempfile :: TempDir ;}
mkitem!{mkstruct!{# [doc = " This is used to avoid TempDir being dropped on error paths unintentionally."] # [derive (Debug)] pub struct MaybeTempDir { dir : ManuallyDrop < TempDir > , keep : bool , }}}
mkitem!{mkimpl!{impl Drop for MaybeTempDir { fn drop (& mut self) { let dir = unsafe { ManuallyDrop :: take (& mut self . dir) } ; if self . keep { let _ = dir . keep () ; } } }}}
mkitem!{mkimpl!{impl AsRef < Path > for MaybeTempDir { fn as_ref (& self) -> & Path { self . dir . path () } }}}
mkitem!{mkimpl!{impl MaybeTempDir { pub fn new (dir : TempDir , keep_on_drop : bool) -> MaybeTempDir { MaybeTempDir { dir : ManuallyDrop :: new (dir) , keep : keep_on_drop } } }}}