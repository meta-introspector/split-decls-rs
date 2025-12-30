// Generated macro for impl_32 (impl)
macro_rules! Depcrate_block_apiimpl_32 {
() => {
// Module: crate::block_api
// Provides: {"impl_32"}
// Dependencies: {}
impl < P : Gost94Params > SerializableState for Gost94Core < P > { type SerializedStateSize = U96 ; fn serialize (& self) -> SerializedState < Self > { let serialized_h = Array :: < _ , U32 > :: from (self . h) ; let mut serialized_n = Array :: < _ , U32 > :: default () ; for (val , chunk) in self . n . iter () . zip (serialized_n . chunks_exact_mut (8)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } let mut serialized_sigma = Array :: < _ , U32 > :: default () ; for (val , chunk) in self . sigma . iter () . zip (serialized_sigma . chunks_exact_mut (8)) { chunk . copy_from_slice (& val . to_le_bytes ()) ; } serialized_h . concat (serialized_n) . concat (serialized_sigma) } fn deserialize (serialized_state : & SerializedState < Self > ,) -> Result < Self , DeserializeStateError > { let (serialized_h , remaining_buffer) = serialized_state . split :: < U32 > () ; let (serialized_n , serialized_sigma) = remaining_buffer . split :: < U32 > () ; let mut n = [0 ; 4] ; for (val , chunk) in n . iter_mut () . zip (serialized_n . chunks_exact (8)) { * val = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } let mut sigma = [0 ; 4] ; for (val , chunk) in sigma . iter_mut () . zip (serialized_sigma . chunks_exact (8)) { * val = u64 :: from_le_bytes (chunk . try_into () . unwrap ()) ; } Ok (Self { h : serialized_h . into () , n , sigma , _m : core :: marker :: PhantomData , }) } }
};
}
