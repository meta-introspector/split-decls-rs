// Generated macro for Ty (enum)
macro_rules! Depcrate_rust_typeTy {
() => {
// Module: crate::rust_type
// Provides: {"Ty"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum Ty { Primitive (Primitive) , Pointee (PointeeTy) , # [doc = " Only handle vectors specially, matrixes are \"just\" structs on top."] Simd { ty : Primitive , size : u8 , } , Sel { nullability : Nullability , } , Pointer { nullability : Nullability , # [doc = " Whether this pointer may be read through."] read : bool , # [doc = " Whether this pointer may be written to."] written : bool , lifetime : Lifetime , bounds : PointerBounds , pointee : Box < Self > , } , TypeDef { id : ItemIdentifier , to : Box < Self > , } , Array { element_type : Box < Self > , num_elements : usize , } , Enum { id : ItemIdentifier , ty : Box < Self > , } , Struct { id : ItemIdentifier , # [doc = " FIXME: This does not work for recursive structs."] fields : Vec < Ty > , # [doc = " Whether the struct's declaration has a bridge attribute."] is_bridged : bool , } , Union { id : ItemIdentifier , # [doc = " FIXME: This does not work for recursive structs."] fields : Vec < Ty > , } , }
};
}
