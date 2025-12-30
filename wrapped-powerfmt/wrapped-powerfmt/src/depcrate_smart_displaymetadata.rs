// Generated macro for Metadata (struct)
macro_rules! Depcrate_smart_displayMetadata {
() => {
// Module: crate::smart_display
// Provides: {"Metadata"}
// Dependencies: {}
# [doc = " Information used to format a value. This is returned by [`SmartDisplay::metadata`]."] # [doc = ""] # [doc = " This type is generic over any user-provided type. This allows the author to store any"] # [doc = " information that is needed. For example, a type's implementation of [`SmartDisplay`] may need"] # [doc = " to calculate something before knowing its width. This calculation can be performed, with the"] # [doc = " result being stored in the custom metadata type."] # [doc = ""] # [doc = " Note that `Metadata` _always_ contains the width of the type. Authors do not need to store this"] # [doc = " information in their custom metadata type."] # [doc = ""] # [doc = " Generally speaking, a type should be able to be formatted using only its metadata, fields, and"] # [doc = " the formatter. Any other information should be stored in the metadata type."] pub struct Metadata < 'a , T > where T : SmartDisplay + ? Sized , { unpadded_width : usize , metadata : T :: Metadata , _value : PhantomData < & 'a T > , }
};
}
