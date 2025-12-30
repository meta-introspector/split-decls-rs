// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a , SomePart , Target , FieldType > Mut < SomePart , Ref < 'a , Target > > where FieldType : ? Sized , SomePart : Part < PartType = Field < FieldType > > , Target : ? Sized , Target : HasPart < SomePart > , { # [doc = " Only available on single part references, used to implement the more general"] # [doc = " [`part_mut`](PartialRef::part_mut) method of [`PartialRef`]"] # [inline (always)] fn get_part_mut (self) -> & 'a mut FieldType { unsafe { & mut * Target :: part_ptr_mut (self . get_raw ()) } } }
};
}
