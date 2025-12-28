macro_rules! deps {
    () => {
        Type!();
        Param!();
        Function!();
        Local!();
        Closure!();
        Callee!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < 'db > Param < 'db > { pub fn parent_fn (& self) -> Option < Function > { match self . func { Callee :: Def (CallableDefId :: FunctionId (f)) => Some (f . into ()) , _ => None , } } pub fn index (& self) -> usize { self . idx } pub fn ty (& self) -> & Type < 'db > { & self . ty } pub fn name (& self , db : & dyn HirDatabase) -> Option < Name > { Some (self . as_local (db) ? . name (db)) } pub fn as_local (& self , db : & dyn HirDatabase) -> Option < Local > { match self . func { Callee :: Def (CallableDefId :: FunctionId (it)) => { let parent = DefWithBodyId :: FunctionId (it) ; let body = db . body (parent) ; if let Some (self_param) = body . self_param . filter (| _ | self . idx == 0) { Some (Local { parent , binding_id : self_param }) } else if let Pat :: Bind { id , .. } = & body [body . params [self . idx - body . self_param . is_some () as usize]] { Some (Local { parent , binding_id : * id }) } else { None } } Callee :: Closure (closure , _) => { let c = db . lookup_intern_closure (closure) ; let body = db . body (c . 0) ; if let Expr :: Closure { args , .. } = & body [c . 1] && let Pat :: Bind { id , .. } = & body [args [self . idx]] { return Some (Local { parent : c . 0 , binding_id : * id }) ; } None } _ => None , } } pub fn pattern_source (self , db : & dyn HirDatabase) -> Option < ast :: Pat > { self . source (db) . and_then (| p | p . value . right () ? . pat ()) } }
    };
}

impl_289!()