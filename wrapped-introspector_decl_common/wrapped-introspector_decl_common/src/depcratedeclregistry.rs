// Generated macro for DeclRegistry (struct)
macro_rules! DepcrateDeclRegistry {
() => {
// Module: crate
// Provides: {"DeclRegistry"}
// Dependencies: {}
# [derive (Debug , Clone , Default)] pub struct DeclRegistry { pub declarations : Vec < DeclInfo > , pub by_type : HashMap < String , Vec < usize > > , pub by_module : HashMap < String , Vec < usize > > , pub by_hash : HashMap < String , usize > , }
};
}
