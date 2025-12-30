// Generated macro for parts (module)
macro_rules! Depcrate_list_formatterparts {
() => {
// Module: crate::list_formatter
// Provides: {"parts"}
// Dependencies: {}
# [doc = " The [`Part`]s used by [`ListFormatter`]."] pub mod parts { use writeable :: Part ; # [doc = " The [`Part`] used by [`FormattedList`](super::FormattedList) to mark the part of the string that is an element."] # [doc = ""] # [doc = " * `category`: `\"list\"`"] # [doc = " * `value`: `\"element\"`"] pub const ELEMENT : Part = Part { category : "list" , value : "element" , } ; # [doc = " The [`Part`] used by [`FormattedList`](super::FormattedList) to mark the part of the string that is a list literal,"] # [doc = " such as \", \" or \" and \"."] # [doc = ""] # [doc = " * `category`: `\"list\"`"] # [doc = " * `value`: `\"literal\"`"] pub const LITERAL : Part = Part { category : "list" , value : "literal" , } ; }
};
}
