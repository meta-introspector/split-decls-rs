// Generated macro for msgitem_convert (macro)
macro_rules! Depcrate_arg_messageitemmsgitem_convert {
() => {
// Module: crate::arg::messageitem
// Provides: {"msgitem_convert"}
// Dependencies: {}
macro_rules ! msgitem_convert { ($ t : ty , $ s : ident) => { impl From <$ t > for MessageItem { fn from (i : $ t) -> MessageItem { MessageItem ::$ s (i) } } impl <'a > TryFrom <&'a MessageItem > for $ t { type Error = () ; fn try_from (i : &'a MessageItem) -> Result <$ t , () > { if let MessageItem ::$ s (b) = i . peel () { Ok (* b) } else { Err (()) } } } } }
};
}
