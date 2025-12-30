// Generated macro for impl_23 (impl)
macro_rules! Depcrate_messageimpl_23 {
() => {
// Module: crate::message
// Provides: {"impl_23"}
// Dependencies: {}
impl fmt :: Debug for Message { fn fmt (& self , f : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { let mut x = f . debug_struct ("Message") ; x . field ("Type" , & self . msg_type ()) ; if let Some (ref path) = self . path () { x . field ("Path" , & & * * path) ; } if let Some (ref iface) = self . interface () { x . field ("Interface" , & & * * iface) ; } if let Some (ref member) = self . member () { x . field ("Member" , & & * * member) ; } if let Some (ref sender) = self . sender () { x . field ("Sender" , & & * * sender) ; } if let Some (ref dest) = self . destination () { x . field ("Destination" , & & * * dest) ; } if let Some (ref serial) = self . get_serial () { x . field ("Serial" , serial) ; } if let Some (ref rs) = self . get_reply_serial () { x . field ("ReplySerial" , rs) ; } let mut args = vec ! () ; let mut iter = self . iter_init () ; while let Some (a) = iter . get_refarg () { args . push (a) ; iter . next () ; } let args2 : & [_] = & args ; x . field ("Args" , & args2) ; x . finish () } }
};
}
