// Generated macro for ArrayStorage (struct)
macro_rules! Depcrate_base_array_storageArrayStorage {
() => {
// Module: crate::base::array_storage
// Provides: {"ArrayStorage"}
// Dependencies: {}
# [doc = " An array-based statically sized matrix data storage."] # [repr (transparent)] # [derive (Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "ArrayStorage<T::Archived, R, C>" , bound (archive = "
        T: rkyv::Archive,
        [[T; R]; C]: rkyv::Archive<Archived = [[T::Archived; R]; C]>
    ")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] pub struct ArrayStorage < T , const R : usize , const C : usize > (pub [[T ; R] ; C]) ;
};
}
