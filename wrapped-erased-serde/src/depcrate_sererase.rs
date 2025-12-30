// Generated macro for erase (module)
macro_rules! Depcrate_sererase {
() => {
// Module: crate::ser
// Provides: {"erase"}
// Dependencies: {}
mod erase { use core :: mem ; pub enum Serializer < S > where S : serde :: Serializer , { Ready (S) , Seq (S :: SerializeSeq) , Tuple (S :: SerializeTuple) , TupleStruct (S :: SerializeTupleStruct) , TupleVariant (S :: SerializeTupleVariant) , Map (S :: SerializeMap) , Struct (S :: SerializeStruct) , StructVariant (S :: SerializeStructVariant) , Error (S :: Error) , Complete (S :: Ok) , Unusable , } impl < S > Serializer < S > where S : serde :: Serializer , { pub (crate) fn new (serializer : S) -> Self { Serializer :: Ready (serializer) } pub (crate) fn take (& mut self) -> Self { mem :: replace (self , Serializer :: Unusable) } pub (crate) fn take_serializer (& mut self) -> S { match self . take () { Serializer :: Ready (serializer) => serializer , _ => unreachable ! () , } } } }
};
}
