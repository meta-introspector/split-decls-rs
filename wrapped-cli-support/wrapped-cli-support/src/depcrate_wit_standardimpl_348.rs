// Generated macro for impl_348 (impl)
macro_rules! Depcrate_wit_standardimpl_348 {
() => {
// Module: crate::wit::standard
// Provides: {"impl_348"}
// Dependencies: {}
impl NonstandardWitSection { pub fn append (& mut self , params : Vec < AdapterType > , results : Vec < AdapterType > , inner_results : Vec < AdapterType > , kind : AdapterKind ,) -> AdapterId { let id = AdapterId (self . adapters . len ()) ; self . adapters . insert (id , Adapter { id , params , results , inner_results , kind , } ,) ; id } # [doc = " Removes any dead entries in `adapters` that are no longer necessary"] # [doc = " and/or no longer referenced."] # [doc = ""] # [doc = " Returns `true` if any adapters were deleted, or `false` if the adapters"] # [doc = " did not change."] pub fn gc (& mut self) -> bool { let mut live = HashSet :: new () ; for (_ , id) in self . exports . iter () { self . add_live (* id , & mut live) ; } for (_ , _ , id) in self . implements . iter () { self . add_live (* id , & mut live) ; } let before = self . adapters . len () ; self . adapters . retain (| id , _ | live . contains (id)) ; before != self . adapters . len () } fn add_live (& self , id : AdapterId , live : & mut HashSet < AdapterId >) { if ! live . insert (id) { return ; } let instructions = match & self . adapters [& id] . kind { AdapterKind :: Local { instructions } => instructions , AdapterKind :: Import { .. } => return , } ; for instr in instructions { match instr . instr { Instruction :: Closure { adapter , .. } | Instruction :: CallAdapter (adapter) => { self . add_live (adapter , live) ; } _ => { } } } } }
};
}
