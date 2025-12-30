// Generated macro for impl_654 (impl)
macro_rules! Depcrate_base_array_storageimpl_654 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_654"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T , const R : usize , const C : usize > Visitor < 'a > for ArrayStorageVisitor < T , R , C > where T : Scalar + Deserialize < 'a > , { type Value = ArrayStorage < T , R , C > ; fn expecting (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a matrix array") } # [inline] fn visit_seq < V > (self , mut visitor : V) -> Result < ArrayStorage < T , R , C > , V :: Error > where V : SeqAccess < 'a > , { let mut out : ArrayStorage < core :: mem :: MaybeUninit < T > , R , C > = < DefaultAllocator as Allocator < _ , _ > > :: allocate_uninit (Const :: < R > , Const :: < C >) ; let mut curr = 0 ; while let Some (value) = visitor . next_element () ? { * out . as_mut_slice () . get_mut (curr) . ok_or_else (| | V :: Error :: invalid_length (curr , & self)) ? = core :: mem :: MaybeUninit :: new (value) ; curr += 1 ; } if curr == R * C { unsafe { Ok (< DefaultAllocator as Allocator < Const < R > , Const < C > > > :: assume_init (out)) } } else { for i in 0 .. curr { unsafe { std :: ptr :: drop_in_place (out . as_mut_slice () [i] . as_mut_ptr ()) } ; } Err (V :: Error :: invalid_length (curr , & self)) } } }
};
}
