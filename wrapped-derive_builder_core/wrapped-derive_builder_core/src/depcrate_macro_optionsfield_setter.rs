// Generated macro for field_setter (function)
macro_rules! Depcrate_macro_optionsfield_setter {
() => {
// Module: crate::macro_options
// Provides: {"field_setter"}
// Dependencies: {}
# [doc = " `derive_builder` allows the calling code to use `setter` as a word to enable"] # [doc = " setters when they've been disabled at the struct level."] fn field_setter (meta : & Meta) -> darling :: Result < FieldLevelSetter > { if let Meta :: Path (_) = meta { Ok (FieldLevelSetter { skip : Some (false) , .. Default :: default () }) } else { FieldLevelSetter :: from_meta (meta) } }
};
}
