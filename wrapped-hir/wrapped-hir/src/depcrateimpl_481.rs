// Generated macro for impl_481 (impl)
macro_rules! Depcrateimpl_481 {
() => {
// Module: crate
// Provides: {"impl_481"}
// Dependencies: {}
impl < 'db > Callable < 'db > { pub fn kind (& self) -> CallableKind { match self . callee { Callee :: Def (CallableDefId :: FunctionId (it)) => CallableKind :: Function (it . into ()) , Callee :: Def (CallableDefId :: StructId (it)) => CallableKind :: TupleStruct (it . into ()) , Callee :: Def (CallableDefId :: EnumVariantId (it)) => { CallableKind :: TupleEnumVariant (it . into ()) } Callee :: Closure (id , ref subst) => { CallableKind :: Closure (Closure { id , subst : subst . clone () }) } Callee :: FnPtr => CallableKind :: FnPtr , Callee :: FnImpl (fn_) => CallableKind :: FnImpl (fn_) , } } pub fn receiver_param (& self , db : & 'db dyn HirDatabase) -> Option < (SelfParam , Type < 'db >) > { let func = match self . callee { Callee :: Def (CallableDefId :: FunctionId (it)) if self . is_bound_method => it , _ => return None , } ; let func = Function { id : func } ; Some ((func . self_param (db) ? , self . ty . derived (self . sig . params () [0] . clone ()))) } pub fn n_params (& self) -> usize { self . sig . params () . len () - if self . is_bound_method { 1 } else { 0 } } pub fn params (& self) -> Vec < Param < 'db > > { self . sig . params () . iter () . enumerate () . skip (if self . is_bound_method { 1 } else { 0 }) . map (| (idx , ty) | (idx , self . ty . derived (ty . clone ()))) . map (| (idx , ty) | Param { func : self . callee . clone () , idx , ty }) . collect () } pub fn return_type (& self) -> Type < 'db > { self . ty . derived (self . sig . ret () . clone ()) } pub fn sig (& self) -> & CallableSig { & self . sig } pub fn ty (& self) -> & Type < 'db > { & self . ty } }
};
}
