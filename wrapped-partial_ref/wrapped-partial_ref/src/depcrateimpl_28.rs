// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a , SomePart , Target , FieldType > Const < SomePart , Ref < 'a , Target > > where FieldType : ? Sized , SomePart : Part < PartType = Field < FieldType > > , Target : ? Sized , Target : HasPart < SomePart > , { # [doc = " Only available on single part references, used to implement the more general"] # [doc = " [`part`](PartialRef::part) method of [`PartialRef`]."] # [inline (always)] fn get_part (self) -> & 'a FieldType { unsafe { & * Target :: part_ptr (self . get_raw () as * const _) } } }
};
}
