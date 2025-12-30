// Generated macro for frame_types (macro)
macro_rules! Depcrate_frameframe_types {
() => {
// Module: crate::frame
// Provides: {"frame_types"}
// Dependencies: {}
macro_rules ! frame_types { { $ ($ name : ident = $ val : expr ,) * } => { impl FrameType { $ (pub (crate) const $ name : FrameType = FrameType ($ val) ;) * } impl fmt :: Debug for FrameType { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { match self . 0 { $ ($ val => f . write_str (stringify ! ($ name)) ,) * _ => write ! (f , "Type({:02x})" , self . 0) } } } impl fmt :: Display for FrameType { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { match self . 0 { $ ($ val => f . write_str (stringify ! ($ name)) ,) * x if STREAM_TYS . contains (& x) => f . write_str ("STREAM") , x if DATAGRAM_TYS . contains (& x) => f . write_str ("DATAGRAM") , _ => write ! (f , "<unknown {:02x}>" , self . 0) , } } } } }
};
}
