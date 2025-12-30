// Generated macro for WriteTo (trait)
macro_rules! Depcrate_traitsWriteTo {
() => {
// Module: crate::traits
// Provides: {"WriteTo"}
// Dependencies: {}
# [doc = " Writing of objects to a `Write` implementation"] pub trait WriteTo { # [doc = " Write a representation of this instance to `out`."] fn write_to (& self , out : & mut dyn std :: io :: Write) -> std :: io :: Result < () > ; # [doc = " Returns the type of this object."] fn kind (& self) -> Kind ; # [doc = " Returns the size of this object's representation (the amount"] # [doc = " of data which would be written by [`write_to`](Self::write_to))."] # [doc = ""] # [doc = " [`size`](Self::size)'s value has no bearing on the validity of"] # [doc = " the object, as such it's possible for [`size`](Self::size) to"] # [doc = " return a sensible value but [`write_to`](Self::write_to) to"] # [doc = " fail because the object was not actually valid in some way."] fn size (& self) -> u64 ; # [doc = " Returns a loose object header based on the object's data"] fn loose_header (& self) -> smallvec :: SmallVec < u8 , 28 > { crate :: encode :: loose_header (self . kind () , self . size ()) } }
};
}
