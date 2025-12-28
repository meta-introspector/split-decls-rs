macro_rules! deps {
    () => {
        Size!();
        Scalar!();
        Primitive!();
    };
}

macro_rules! FieldsShape {
    () => {
        deps!();
        # [doc = " Describes how the fields of a type are located in memory."] # [derive (PartialEq , Eq , Hash , Clone , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_Generic))] pub enum FieldsShape < FieldIdx : Idx > { # [doc = " Scalar primitives and `!`, which never have fields."] Primitive , # [doc = " All fields start at no offset. The `usize` is the field count."] Union (NonZeroUsize) , # [doc = " Array/vector-like placement, with all fields of identical types."] Array { stride : Size , count : u64 } , # [doc = " Struct-like placement, with precomputed offsets."] # [doc = ""] # [doc = " Fields are guaranteed to not overlap, but note that gaps"] # [doc = " before, between and after all the fields are NOT always"] # [doc = " padding, and as such their contents may not be discarded."] # [doc = " For example, enum variants leave a gap at the start,"] # [doc = " where the discriminant field in the enum layout goes."] Arbitrary { # [doc = " Offsets for the first byte of each field,"] # [doc = " ordered to match the source definition order."] # [doc = " This vector does not go in increasing order."] offsets : IndexVec < FieldIdx , Size > , # [doc = " Maps source order field indices to memory order indices,"] # [doc = " depending on how the fields were reordered (if at all)."] # [doc = " This is a permutation, with both the source order and the"] # [doc = " memory order using the same (0..n) index ranges."] # [doc = ""] # [doc = " Note that during computation of `memory_index`, sometimes"] # [doc = " it is easier to operate on the inverse mapping (that is,"] # [doc = " from memory order to source order), and that is usually"] # [doc = " named `inverse_memory_index`."] # [doc = ""] memory_index : IndexVec < FieldIdx , u32 > , } , }
    };
}

FieldsShape!()