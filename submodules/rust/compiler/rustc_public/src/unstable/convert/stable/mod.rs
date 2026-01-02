mkuse!{use rustc_abi :: FieldIdx ;}
mkuse!{use rustc_public_bridge :: Tables ;}
mkuse!{use rustc_public_bridge :: context :: CompilerCtxt ;}
mkuse!{use super :: Stable ;}
mkuse!{use crate :: compiler_interface :: BridgeTys ;}
mkmod!{abi, { 
                getname!(abi);
                getsrc!(abi);
                getpath!(abi);
                get_deps!(abi);
                get_crates!(abi);
                mkinclude!(abi);
                 
            }}
mkmod!{mir, { 
                getname!(mir);
                getsrc!(mir);
                getpath!(mir);
                get_deps!(mir);
                get_crates!(mir);
                mkinclude!(mir);
                 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                 
            }}
mkitem!{mkimpl!{impl < 'tcx > Stable < 'tcx > for rustc_hir :: Safety { type T = crate :: mir :: Safety ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { rustc_hir :: Safety :: Unsafe => crate :: mir :: Safety :: Unsafe , rustc_hir :: Safety :: Safe => crate :: mir :: Safety :: Safe , } } }}}
mkitem!{mkimpl!{impl < 'tcx > Stable < 'tcx > for FieldIdx { type T = usize ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { self . as_usize () } }}}
mkitem!{mkimpl!{impl < 'tcx > Stable < 'tcx > for rustc_hir :: CoroutineSource { type T = crate :: mir :: CoroutineSource ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_hir :: CoroutineSource ; match self { CoroutineSource :: Block => crate :: mir :: CoroutineSource :: Block , CoroutineSource :: Closure => crate :: mir :: CoroutineSource :: Closure , CoroutineSource :: Fn => crate :: mir :: CoroutineSource :: Fn , } } }}}
mkitem!{mkimpl!{impl < 'tcx > Stable < 'tcx > for rustc_hir :: CoroutineKind { type T = crate :: mir :: CoroutineKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_hir :: { CoroutineDesugaring , CoroutineKind } ; match * self { CoroutineKind :: Desugared (CoroutineDesugaring :: Async , source) => { crate :: mir :: CoroutineKind :: Desugared (crate :: mir :: CoroutineDesugaring :: Async , source . stable (tables , cx) ,) } CoroutineKind :: Desugared (CoroutineDesugaring :: Gen , source) => { crate :: mir :: CoroutineKind :: Desugared (crate :: mir :: CoroutineDesugaring :: Gen , source . stable (tables , cx) ,) } CoroutineKind :: Coroutine (movability) => { crate :: mir :: CoroutineKind :: Coroutine (movability . stable (tables , cx)) } CoroutineKind :: Desugared (CoroutineDesugaring :: AsyncGen , source) => { crate :: mir :: CoroutineKind :: Desugared (crate :: mir :: CoroutineDesugaring :: AsyncGen , source . stable (tables , cx) ,) } } } }}}
mkitem!{mkimpl!{impl < 'tcx > Stable < 'tcx > for rustc_span :: Symbol { type T = crate :: Symbol ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { self . to_string () } }}}
mkitem!{mkimpl!{impl < 'tcx > Stable < 'tcx > for rustc_span :: Span { type T = crate :: ty :: Span ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , _ : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . create_span (* self) } }}}