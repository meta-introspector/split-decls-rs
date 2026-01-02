mkmod!{bug, { 
                getname!(bug);
                getsrc!(bug);
                getpath!(bug);
                get_deps!(bug);
                get_crates!(bug);
                mkinclude!(bug);
                 
            }}
mkitem!{mkstruct!{# [derive (Default , Copy , Clone)] pub struct Providers { pub queries : crate :: query :: Providers , pub extern_queries : crate :: query :: ExternProviders , pub hooks : crate :: hooks :: Providers , }}}
mkitem!{mkimpl!{# [doc = " Backwards compatibility hack to keep the diff small. This"] # [doc = " gives direct access to the `queries` field's fields, which"] # [doc = " are what almost everything wants access to."] impl std :: ops :: DerefMut for Providers { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . queries } }}}
mkitem!{mkimpl!{impl std :: ops :: Deref for Providers { type Target = crate :: query :: Providers ; fn deref (& self) -> & Self :: Target { & self . queries } }}}