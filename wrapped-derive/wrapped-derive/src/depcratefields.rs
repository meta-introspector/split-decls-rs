// Generated macro for fields (function)
macro_rules! Depcratefields {
() => {
// Module: crate
// Provides: {"fields"}
// Dependencies: {}
fn fields (input : & DeriveInput) -> Result < & Fields > { use syn :: Fields ; match & input . data { Data :: Struct (data) => match & data . fields { Fields :: Named (fields) => Ok (& fields . named) , Fields :: Unnamed (fields) => Ok (& fields . unnamed) , Fields :: Unit => Err (Error :: new (Span :: call_site () , "RefCast does not support unit structs" ,)) , } , Data :: Enum (_) => Err (Error :: new (Span :: call_site () , "RefCast does not support enums" ,)) , Data :: Union (_) => Err (Error :: new (Span :: call_site () , "RefCast does not support unions" ,)) , } }
};
}
