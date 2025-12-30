// Generated macro for MirBody (struct)
macro_rules! Depcrate_mirMirBody {
() => {
// Module: crate::mir
// Provides: {"MirBody"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq)] pub struct MirBody < 'db > { pub projection_store : ProjectionStore < 'db > , pub basic_blocks : Arena < BasicBlock < 'db > > , pub locals : Arena < Local < 'db > > , pub start_block : BasicBlockId < 'db > , pub owner : DefWithBodyId , pub binding_locals : ArenaMap < BindingId , LocalId < 'db > > , pub param_locals : Vec < LocalId < 'db > > , # [doc = " This field stores the closures directly owned by this body. It is used"] # [doc = " in traversing every mir body."] pub closures : Vec < InternedClosureId > , }
};
}
