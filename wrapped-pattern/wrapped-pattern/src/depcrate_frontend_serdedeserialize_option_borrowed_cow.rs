// Generated macro for deserialize_option_borrowed_cow (function)
macro_rules! Depcrate_frontend_serdedeserialize_option_borrowed_cow {
() => {
// Module: crate::frontend::serde
// Provides: {"deserialize_option_borrowed_cow"}
// Dependencies: {}
# [doc (hidden)] pub fn deserialize_option_borrowed_cow < 'de , 'data , B , D : Deserializer < 'de > > (deserializer : D ,) -> Result < Option < Cow < 'data , Pattern < B > > > , D :: Error > where 'de : 'data , B : PatternBackend < Store = str > , B :: PlaceholderKeyCow < 'data > : Deserialize < 'de > , & 'data B :: Store : Deserialize < 'de > , { # [derive (Deserialize)] # [serde (transparent)] struct CowPatternWrap < 'data1 , B : PatternBackend < Store = str > > where Box < B :: Store > : for < 'a > From < & 'a B :: Store > , { # [serde (borrow , deserialize_with = "deserialize_borrowed_cow::<B, _>" , bound = "B::PlaceholderKeyCow<'data1>: Deserialize<'de>, &'data1 B::Store: Deserialize<'de>")] pub cow : Cow < 'data1 , Pattern < B > > , } Option :: < CowPatternWrap < 'data , B > > :: deserialize (deserializer) . map (| option | option . map (| wrap | wrap . cow)) }
};
}
