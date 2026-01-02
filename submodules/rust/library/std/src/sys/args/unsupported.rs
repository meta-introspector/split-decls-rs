mkuse!{use crate :: ffi :: OsString ;}
mkuse!{use crate :: fmt ;}
mkitem!{mkstruct!{pub struct Args { }}}

macro_rules! args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function args in module {}", module_path!());
    };
}

mkfn!{
    args_introspect!();
    pub fn args () -> Args { Args { } }
}
mkitem!{mkimpl!{impl fmt :: Debug for Args { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . finish () } }}}
mkitem!{mkimpl!{impl Iterator for Args { type Item = OsString ; # [inline] fn next (& mut self) -> Option < OsString > { None } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }}}
mkitem!{mkimpl!{impl DoubleEndedIterator for Args { # [inline] fn next_back (& mut self) -> Option < OsString > { None } }}}
mkitem!{mkimpl!{impl ExactSizeIterator for Args { # [inline] fn len (& self) -> usize { 0 } }}}