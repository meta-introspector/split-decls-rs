// Generated macro for QSelf (struct)
macro_rules! Depcrate_astQSelf {
() => {
// Module: crate::ast
// Provides: {"QSelf"}
// Dependencies: {}
# [doc = " The explicit `Self` type in a \"qualified path\". The actual"] # [doc = " path, including the trait and the associated item, is stored"] # [doc = " separately. `position` represents the index of the associated"] # [doc = " item qualified with this `Self` type."] # [doc = ""] # [doc = " ```ignore (only-for-syntax-highlight)"] # [doc = " <Vec<T> as a::b::Trait>::AssociatedItem"] # [doc = "  ^~~~~     ~~~~~~~~~~~~~~^"] # [doc = "  ty        position = 3"] # [doc = ""] # [doc = " <Vec<T>>::AssociatedItem"] # [doc = "  ^~~~~    ^"] # [doc = "  ty       position = 0"] # [doc = " ```"] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct QSelf { pub ty : Box < Ty > , # [doc = " The span of `a::b::Trait` in a path like `<Vec<T> as"] # [doc = " a::b::Trait>::AssociatedItem`; in the case where `position =="] # [doc = " 0`, this is an empty span."] pub path_span : Span , pub position : usize , }
};
}
