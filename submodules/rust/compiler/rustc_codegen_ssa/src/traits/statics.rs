mkuse!{use rustc_abi :: Align ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use super :: BackendTypes ;}
mkitem!{mktrait!{pub trait StaticCodegenMethods : BackendTypes { fn static_addr_of (& self , cv : Self :: Value , align : Align , kind : Option < & str >) -> Self :: Value ; fn codegen_static (& mut self , def_id : DefId) ; }}}
mkitem!{mktrait!{pub trait StaticBuilderMethods : BackendTypes { fn get_static (& mut self , def_id : DefId) -> Self :: Value ; }}}