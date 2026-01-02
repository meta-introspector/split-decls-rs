mkuse!{use std :: ops :: RangeInclusive ;}
mkuse!{use rustc_public_bridge :: Tables ;}
mkuse!{use rustc_public_bridge :: context :: CompilerCtxt ;}
mkuse!{use super :: Stable ;}
mkuse!{use crate :: compiler_interface :: BridgeTys ;}
mkmod!{internal, { 
                getname!(internal);
                getsrc!(internal);
                getpath!(internal);
                get_deps!(internal);
                get_crates!(internal);
                mkinclude!(internal);
                 
            }}
mkmod!{stable, { 
                getname!(stable);
                getsrc!(stable);
                getpath!(stable);
                get_deps!(stable);
                get_crates!(stable);
                mkinclude!(stable);
                 
            }}
mkitem!{mkimpl!{impl < 'tcx , T > Stable < 'tcx > for & T where T : Stable < 'tcx > , { type T = T :: T ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { (* self) . stable (tables , cx) } }}}
mkitem!{mkimpl!{impl < 'tcx , T > Stable < 'tcx > for Option < T > where T : Stable < 'tcx > , { type T = Option < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . as_ref () . map (| value | value . stable (tables , cx)) } }}}
mkitem!{mkimpl!{impl < 'tcx , T , E > Stable < 'tcx > for Result < T , E > where T : Stable < 'tcx > , E : Stable < 'tcx > , { type T = Result < T :: T , E :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { Ok (val) => Ok (val . stable (tables , cx)) , Err (error) => Err (error . stable (tables , cx)) , } } }}}
mkitem!{mkimpl!{impl < 'tcx , T > Stable < 'tcx > for & [T] where T : Stable < 'tcx > , { type T = Vec < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . iter () . map (| e | e . stable (tables , cx)) . collect () } }}}
mkitem!{mkimpl!{impl < 'tcx , T , U > Stable < 'tcx > for (T , U) where T : Stable < 'tcx > , U : Stable < 'tcx > , { type T = (T :: T , U :: T) ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { (self . 0 . stable (tables , cx) , self . 1 . stable (tables , cx)) } }}}
mkitem!{mkimpl!{impl < 'tcx , T > Stable < 'tcx > for RangeInclusive < T > where T : Stable < 'tcx > , { type T = RangeInclusive < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { RangeInclusive :: new (self . start () . stable (tables , cx) , self . end () . stable (tables , cx)) } }}}