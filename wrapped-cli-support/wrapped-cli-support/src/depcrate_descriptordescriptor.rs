// Generated macro for Descriptor (enum)
macro_rules! Depcrate_descriptorDescriptor {
() => {
// Module: crate::descriptor
// Provides: {"Descriptor"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum Descriptor { I8 , U8 , ClampedU8 , I16 , U16 , I32 , U32 , I64 , U64 , I128 , U128 , F32 , F64 , Boolean , Function (Box < Function >) , Closure (Box < Closure >) , Ref (Box < Descriptor >) , RefMut (Box < Descriptor >) , Slice (Box < Descriptor >) , Vector (Box < Descriptor >) , CachedString , String , Externref , NamedExternref (String) , Enum { name : String , hole : u32 , } , StringEnum { name : String , invalid : u32 , hole : u32 , } , RustStruct (String) , Char , Option (Box < Descriptor >) , Result (Box < Descriptor >) , Unit , NonNull , }
};
}
