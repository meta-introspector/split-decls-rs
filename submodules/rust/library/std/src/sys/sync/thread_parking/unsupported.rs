mkuse!{use crate :: pin :: Pin ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{mkstruct!{pub struct Parker { }}}
mkitem!{mkimpl!{impl Parker { pub unsafe fn new_in_place (_parker : * mut Parker) { } pub unsafe fn park (self : Pin < & Self >) { } pub unsafe fn park_timeout (self : Pin < & Self > , _dur : Duration) { } pub fn unpark (self : Pin < & Self >) { } }}}