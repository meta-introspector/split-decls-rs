// Generated macro for impl_73 (impl)
macro_rules! Depcrateimpl_73 {
() => {
// Module: crate
// Provides: {"impl_73"}
// Dependencies: {}
impl DiagMessage { # [doc = " Given a `SubdiagMessage` which may contain a Fluent attribute, create a new"] # [doc = " `DiagMessage` that combines that attribute with the Fluent identifier of `self`."] # [doc = ""] # [doc = " - If the `SubdiagMessage` is non-translatable then return the message as a `DiagMessage`."] # [doc = " - If `self` is non-translatable then return `self`'s message."] pub fn with_subdiagnostic_message (& self , sub : SubdiagMessage) -> Self { let attr = match sub { SubdiagMessage :: Str (s) => return DiagMessage :: Str (s) , SubdiagMessage :: Translated (s) => return DiagMessage :: Translated (s) , SubdiagMessage :: FluentIdentifier (id) => { return DiagMessage :: FluentIdentifier (id , None) ; } SubdiagMessage :: FluentAttr (attr) => attr , } ; match self { DiagMessage :: Str (s) => DiagMessage :: Str (s . clone ()) , DiagMessage :: Translated (s) => DiagMessage :: Translated (s . clone ()) , DiagMessage :: FluentIdentifier (id , _) => { DiagMessage :: FluentIdentifier (id . clone () , Some (attr)) } } } pub fn as_str (& self) -> Option < & str > { match self { DiagMessage :: Translated (s) | DiagMessage :: Str (s) => Some (s) , DiagMessage :: FluentIdentifier (_ , _) => None , } } }
};
}
