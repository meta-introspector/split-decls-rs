// Generated macro for PointeeTy (enum)
macro_rules! Depcrate_rust_typePointeeTy {
() => {
// Module: crate::rust_type
// Provides: {"PointeeTy"}
// Dependencies: {}
# [doc = " Types that are only valid behind pointers."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum PointeeTy { Class { id : ItemIdentifier , thread_safety : ThreadSafety , superclasses : Vec < ItemIdentifier > , generics : Vec < PointeeTy > , declaration_generics : Vec < GenericWithBound > , protocols : Vec < (ProtocolRef , ThreadSafety) > , } , GenericParam { name : String , } , AnyObject { protocols : Vec < (ProtocolRef , ThreadSafety) > , } , AnyProtocol , AnyClass { protocols : Vec < (ProtocolRef , ThreadSafety) > , } , Self_ , Fn { is_variadic : bool , no_escape : bool , arguments : Vec < Ty > , result_type : Box < Ty > , } , Block { sendable : Option < bool > , no_escape : bool , arguments : Vec < Ty > , result_type : Box < Ty > , } , CFTypeDef { id : ItemIdentifier , generics : Vec < PointeeTy > , num_declaration_generics : usize , } , CFOpaque , DispatchTypeDef { id : ItemIdentifier , } , TypeDef { id : ItemIdentifier , to : Box < PointeeTy > , } , CStr , }
};
}
