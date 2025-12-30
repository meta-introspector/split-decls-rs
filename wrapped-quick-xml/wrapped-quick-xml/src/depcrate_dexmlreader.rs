// Generated macro for XmlReader (struct)
macro_rules! Depcrate_deXmlReader {
() => {
// Module: crate::de
// Provides: {"XmlReader"}
// Dependencies: {}
# [doc = " An intermediate reader that consumes [`PayloadEvent`]s and produces final [`DeEvent`]s."] # [doc = " [`PayloadEvent::Text`] events, that followed by any event except"] # [doc = " [`PayloadEvent::Text`] or [`PayloadEvent::CData`], are trimmed from the end."] struct XmlReader < 'i , R : XmlRead < 'i > , E : EntityResolver = PredefinedEntityResolver > { # [doc = " A source of low-level XML events"] reader : R , # [doc = " Intermediate event, that could be returned by the next call to `next()`."] # [doc = " If that is the `Text` event then leading spaces already trimmed, but"] # [doc = " trailing spaces is not. Before the event will be returned, trimming of"] # [doc = " the spaces could be necessary"] lookahead : Result < PayloadEvent < 'i > , DeError > , # [doc = " Used to resolve unknown entities that would otherwise cause the parser"] # [doc = " to return an [`EscapeError::UnrecognizedEntity`] error."] # [doc = ""] # [doc = " [`EscapeError::UnrecognizedEntity`]: crate::escape::EscapeError::UnrecognizedEntity"] entity_resolver : E , }
};
}
